use crate::common::Building;
use crate::event::{send_event, try_take_event, Event};
use crate::express::Express;
use crate::slam_club::SlamClub;
use crate::statistics::Statistics;
use crate::talent::{Talent, Talents};
use eframe::egui::{
    CentralPanel, Context, FontData, FontDefinitions, FontFamily, Label, ScrollArea, Sense,
    SidePanel, Ui,
};
use egui_extras::{Column, TableBuilder};
use i18n_utils::{i18n_str, set_lang, Language};
use std::sync::Arc;
use std::time::Duration;
use strum::IntoEnumIterator;

pub struct App {}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let ctx = &cc.egui_ctx;

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "NotoSansSc-Regular".to_string(),
            Arc::new(FontData::from_static(include_bytes!(
                "../NotoSansSC-Regular.otf"
            ))),
        );
        fonts.families.insert(
            FontFamily::Proportional,
            vec!["NotoSansSc-Regular".to_owned()],
        );
        ctx.set_fonts(fonts);

        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let delta = ctx.input(|i| Duration::from_secs_f32(i.stable_dt));

        let statistics = Statistics::single_mut();
        let talents = Talents::single_mut();
        statistics.update(delta);
        while let Some(event) = try_take_event() {
            // println!("handle event: {:?}", event);
            match event {
                Event::DamageResource(source, resource) => {
                    statistics.damage(source, resource);
                }
                Event::CollectResource(source, resource) => {
                    statistics.collect(source, resource);
                }
                Event::Upgrade(upgrade_id) => {
                    Express::single_mut().handle_upgrade(upgrade_id);
                    SlamClub::single_mut().handle_upgrade(upgrade_id);
                }
                Event::ChangeTalent(talent, toggle) => {
                    if toggle {
                        talents.toggle(talent);
                    } else {
                        talents.untoggle(talent);
                    }
                }
            }
        }
        let express = Express::single_mut();
        let slam_club = SlamClub::single_mut();
        express.update(delta);
        slam_club.update(delta);

        let statistics = Statistics::single();
        let express = Express::single();
        let slam_club = SlamClub::single();
        let talents = Talents::single();

        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("English").clicked() {
                    set_lang(Language::en);
                }
                if ui.button("中文").clicked() {
                    set_lang(Language::sc);
                }
            });

            ui.horizontal(|ui| {
                ui.label(&format!("Resource: {}", statistics.resource.ui_string()));
            });

            ui.separator();

            ui.heading("Monitor");
            ui.label("Damage:");
            for (source, value) in statistics.monitor.last_1min_damage.iter() {
                if *value != 0 {
                    ui.label(format!("{source:?}: {value} shards/min"));
                }
            }
            for (source, value) in statistics.monitor.last_5sec_damage.iter() {
                if *value != 0 {
                    ui.label(format!("{source:?}: {value} shards/5sec"));
                }
            }
            ui.label("Collect:");
            for (source, value) in statistics.monitor.last_1min_collect.iter() {
                if *value != 0 {
                    ui.label(format!("{source:?}: {value} shards/min"));
                }
            }
            for (source, value) in statistics.monitor.last_5sec_collect.iter() {
                if *value != 0 {
                    ui.label(format!("{source:?}: {value} shards/5sec"));
                }
            }
        });
        SidePanel::right("right_panel").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for talent in Talent::iter() {
                    let old_toggle = talents.if_toggle(talent);
                    let mut toggle = old_toggle;
                    ui.checkbox(&mut toggle, talent.sc());
                    if toggle != old_toggle {
                        send_event(Event::ChangeTalent(talent, toggle));
                    }
                }
            })
        });
        CentralPanel::default().show(ctx, |ui| {
            building_ui(ui, express);

            ui.separator();

            building_ui(ui, slam_club);
        });

        ctx.request_repaint();
    }
}

fn building_ui<B: Building>(ui: &mut Ui, b: &B) {
    ui.heading(b.name());
    TableBuilder::new(ui)
        .id_salt(b.name())
        .striped(true)
        .sense(Sense::click())
        .column(Column::auto().at_least(80.0))
        .column(Column::auto().at_least(80.0))
        .column(Column::auto().at_least(80.0))
        .body(|mut body| {
            b.upgrade_options(|u| {
                body.row(23.0, |mut row| {
                    row.col(|ui| {
                        ui.add(Label::new(&u.name).selectable(false));
                    });
                    row.col(|ui| {
                        if u.value.level >= 0 {
                            ui.add(Label::new(format!("{}", u.value.level)).selectable(false));
                        }
                    });
                    row.col(|ui| {
                        if u.value.level < 0 && u.value.acquired {
                            ui.add(
                                Label::new(i18n_str!(en=>"Acquired",sc=>"已获得"))
                                    .selectable(false),
                            );
                        } else {
                            // 当 price 为 0 时，并且 value.level 不是 0 时，就表示已经升级到最大值了
                            if u.value.level > 0 && u.price.if_empty() {
                                ui.add(
                                    Label::new(i18n_str!(en=>"Max",sc=>"最大值")).selectable(false),
                                );
                            } else {
                                ui.add(Label::new(&u.price.ui_string()).selectable(false));
                            }
                        }
                    });

                    row.response().on_hover_ui(|ui| {
                        for line in u.description.split('\n') {
                            ui.add(Label::new(line));
                        }
                    });

                    if row.response().clicked() {
                        send_event(Event::Upgrade(u.id));
                    }
                });
            });
        });
}
