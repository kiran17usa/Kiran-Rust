use std::fmt::{self, Formatter, Display};

struct City{
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City{
    fn fmt(&self, f:&mut Formatter)->fmt::Result{
        let lat_c=if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c=if self.lon >= 0.0 {'E'} else {'W'};

        write!(f,"{}: {:.3}{} {:.3}{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(),lon_c)
    }
}

#[derive(Debug)]
struct Color{
    red:u8,
    green:u8,
    blue:u8,
}

fn main()
{
    for city in [
        City{name:"Raleigh", lat:53.345463, lon:-6.2567923},
        City{name:"Durham", lat:59.95, lon:10.75},
        City{name:"Triangle",lat:49.25, lon:-123.1},
    ]
    {
        println!("{}", city);
    }
    for color in [
        Color{red:128, green:255, blue:90},
        Color{red:0, green:3, blue:254},
        Color{red:0, green:0, blue:0},
    ]
    {
        println!("{:?}", color);
    }
}