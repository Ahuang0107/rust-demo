mod trigger_area;

use crate::trigger_area::TriggerArea;

fn main() {
    TriggerArea::new("trigger_area_test_big");
    TriggerArea::new("trigger_area_test_middle");
    TriggerArea::new("trigger_area_test_small");
}
