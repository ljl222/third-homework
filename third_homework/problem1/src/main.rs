enum TrafficLight{
    Red,
    Yellow,
    Green,
}

trait LightTime {
    fn time_for_light(&self) -> u32;
}

impl LightTime for TrafficLight{
    fn time_for_light(&self) -> u32{
        match self{
            TrafficLight::Green => {
                return 50;
            }
            TrafficLight::Red => {
                return 50;
            }
            TrafficLight::Yellow =>{
                return 3;
            }
        }
    }
}

fn main(){
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;
    println!(
        "The red light will light up {:?} seconds \n
        The yellow light will light up {:?} seconds\n
        The green light will light up {:?} seconds\n",
        red.time_for_light(),
        yellow.time_for_light(),
        green.time_for_light()
    );
}