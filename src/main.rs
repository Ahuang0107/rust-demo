use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut tree = Scene::new()
        .with_place_area(
            PlaceArea::new("Ground")
                .with_take_area(TakeArea::new("Item1"))
                .with_take_area(TakeArea::new("Item2")),
        )
        .with_place_area(PlaceArea::new("Shelf").with_take_area(TakeArea::new("Item3")));

    tree.display();
    println!();

    println!("-----测试多次取可变引用以及一次取多个可变引用修改的情况");
    {
        let item_2 = tree.search_take_area("Item2").unwrap();
        item_2.borrow_mut().data.push('A');

        let item_2_again = tree.search_take_area("Item2").unwrap();
        item_2_again.borrow_mut().data.push('B');

        let item_3 = tree.search_take_area("Item3").unwrap();
        item_2.borrow_mut().data.push('C');
        item_3.borrow_mut().data.push('C');
    }
    tree.display();
    println!();

    println!("-----测试同时保留多个Rc的情况");
    {
        tree.hold = Some(Rc::new(RefCell::new(TakeArea::new("Hold"))));
    }
    tree.display();
    println!();
}

#[derive(Debug)]
struct Scene {
    hold: Option<Rc<RefCell<TakeArea>>>,
    place_areas: Vec<PlaceArea>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            hold: None,
            place_areas: Vec::new(),
        }
    }
    pub fn with_place_area(mut self, place_area: PlaceArea) -> Self {
        self.place_areas.push(place_area);
        self
    }
    pub fn search_take_area(&self, name: &str) -> Option<Rc<RefCell<TakeArea>>> {
        for place_area in self.place_areas.iter() {
            if let Some(search_result) = place_area.search_take_area(name) {
                return Some(search_result);
            }
        }
        None
    }
    pub fn display(&self) {
        println!(
            "Scene(Hold: {})",
            if let Some(hold) = &self.hold {
                format!("TA({}-{})", hold.borrow().name, hold.borrow().data)
            } else {
                String::new()
            }
        );
        for place_area in self.place_areas.iter() {
            place_area.display(0);
        }
    }
}

#[derive(Debug)]
struct TakeArea {
    name: &'static str,
    data: String,
    place_areas: Vec<PlaceArea>,
}

impl TakeArea {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            data: String::new(),
            place_areas: Vec::new(),
        }
    }
    pub fn with_place_area(mut self, place_area: PlaceArea) -> Self {
        self.place_areas.push(place_area);
        self
    }
    pub fn search_take_area(&self, name: &str) -> Option<Rc<RefCell<TakeArea>>> {
        for place_area in self.place_areas.iter() {
            if let Some(search_result) = place_area.search_take_area(name) {
                return Some(search_result);
            }
        }
        None
    }
    pub fn display(&self, deep: usize) {
        let spaces = "--".repeat(deep);
        println!("{spaces}TA({}-{})", self.name, self.data);
        for place_area in self.place_areas.iter() {
            place_area.display(deep + 1);
        }
    }
}

#[derive(Debug)]
struct PlaceArea {
    name: &'static str,
    data: String,
    hold: Option<Rc<RefCell<TakeArea>>>,
    take_areas: Vec<Rc<RefCell<TakeArea>>>,
}

impl PlaceArea {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            data: String::new(),
            hold: None,
            take_areas: Vec::new(),
        }
    }
    pub fn with_take_area(mut self, take_area: TakeArea) -> Self {
        self.take_areas.push(Rc::new(RefCell::new(take_area)));
        self
    }
    pub fn search_take_area(&self, name: &str) -> Option<Rc<RefCell<TakeArea>>> {
        for take_area in self.take_areas.iter() {
            if take_area.borrow().name == name {
                return Some(take_area.clone());
            }
            if let Some(search_result) = take_area.borrow().search_take_area(name) {
                return Some(search_result);
            }
        }
        None
    }
    pub fn display(&self, deep: usize) {
        let spaces = "--".repeat(deep);
        println!("{spaces}PA({}-{})", self.name, self.data);
        for take_area in self.take_areas.iter() {
            take_area.borrow().display(deep + 1);
        }
    }
}
