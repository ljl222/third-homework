trait GraphicArea{
    fn get_area(&self) -> u32;
}

fn show_area<T>(graph: &T)
where 
    T: GraphicArea,
{
    let area = graph.get_area();
    println!("The eara of this graph is {:?}",area);
}

struct Square {
    length: u32,
    width: u32,
}

impl GraphicArea for Square {
    fn get_area(&self) -> u32{
        return self.length.checked_mul(self.width).unwrap_or(0);
    }
}

fn main() {
    let gra = Square{ length:25, width: 15};
    show_area(&gra);
}
