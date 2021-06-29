use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct GroundStation{
    radio_freq: f64,
}

fn main() {
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(GroundStation{ radio_freq: 87.64}));

    println!("\nbase1: \nReference counter: {}\nBase: {:?}", Rc::strong_count(&base), base);
    
    {//new scope
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.54;
        println!("\n\nbase1: \nReference counter: {}\nBase: {:?}", Rc::strong_count(&base), base);
        println!("\nbase_2: \nReference counter: The base_2 is RefMut type can't count references\nBase_2: {:?}", base_2);
    }
    
    println!("\n\nbase1: \nReference counter: {}\nBase: {:?}", Rc::strong_count(&base), base);
    
    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 81.4;
    println!("\n\nbase1: \nReference counter: {}\nBase: {:?}", Rc::strong_count(&base), base);
    println!("\nbase_3: \nReference counter: The base_3 is RefMut type can't count references\nBase_3: {:?}", base_3);
    
    drop(base_3);

    println!("\n\nbase1: \nReference counter: {}\nBase: {:?}", Rc::strong_count(&base), base);
}
