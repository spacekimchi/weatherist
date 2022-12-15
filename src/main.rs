use netcdf::extent::Extent;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    println!("hello world");
    let file = netcdf::open("./data/2022.nc")?;
    print_file(&file)?;
    /*
    let lon = &file.variable("longitude").unwrap();
    let o = &lon
        .values_arr::<f32, _>(..)
        .unwrap();
    println!("longitude: {}", o);

    let lat = &file.variable("latitude").unwrap();
    let a = &lat
        .values_arr::<f32, _>(..)
        .unwrap();
    println!("latitude: {}", a);

    let tim = &file.variable("time").unwrap();
    let t = &tim
        .values_arr::<i32, _>(..)
        .unwrap();
    println!("time: {}", t);

    let t2m = &file.variable("t2m").expect("cant find t2m");
    let x = t2m.dimensions()[0].len();
    let y = t2m.dimensions()[1].len();
    let z = t2m.dimensions()[2].len();
    //let val: i16 = t2m.value(0
    println!("x, y, z: {},{},{}", x, y, z);
    println!("t[0], lat, lon: {},{},{}", t[0], a[224], o[965]);
    println!("torrance t2m: {}", t2m.value::<i16, [usize; 3]>([1, 224, 965])? as f32 * 0.0012928243213342896 + 266.2210870952093);

    // torrance: 33.8358, 241.66, (, 60)
    //println!("{:#?}", file.variable("t2m").unwrap());
    //
    // 10m u-component of wind, 10m v-component of wind, 2m dewpoint temperature, 2m temperature, Instantaneous 10m wind gust, Mean total precipitation rate, Skin temperature, Soil temperature level 1, Soil temperature level 2, Total precipitation, Volumetric soil water layer 1, Volumetric soil water layer 2
    */
    Ok(())
}

fn print_file(g: &netcdf::File) -> Result<()> {
    let mut dims = g.dimensions().peekable();
    if dims.peek().is_some() {
        println!("Dimensions:");
        for d in dims {
            if d.is_unlimited() {
                println!("\t{} : Unlimited ({})", d.name(), d.len());
            } else {
                println!("\t{} : ({})", d.name(), d.len());
            }
        }
    }
    let mut types = g.types()?.peekable();
    if types.peek().is_some() {
        println!("Types:");
        for t in types {
            use netcdf::types::VariableType;
            print!("\t{}: ", t.name());
            match t {
                VariableType::Basic(_) | VariableType::String => unreachable!(),
                VariableType::Opaque(o) => println!("Opaque({})", o.size()),
                VariableType::Enum(_) => println!("Enum"),
                VariableType::Vlen(v) => println!("Vlen({})", v.typ().name()),
                VariableType::Compound(c) => {
                    print!("Compound({{");
                    for field in c.fields() {
                        print!(" {}: {} ", field.name(), field.typ().name());
                    }
                    println!("}})");
                }
            }
        }
    }
    let mut variables = g.variables().peekable();
    if variables.peek().is_some() {
        println!("Variables:");
        for v in variables {
            print!("\t{}", v.name());
            print!("(");
            for d in v.dimensions() {
                print!(" {} ", d.name());
            }
            println!("): {}", v.vartype().name());
            for a in v.attributes() {
                println!("\t\t{} = {:?}", a.name(), a.value()?);
            }
        }
    }
    let mut attributes = g.attributes().peekable();
    if attributes.peek().is_some() {
        println!("Attributes:");
        for a in g.attributes() {
            println!("\t\t{} = {:?}", a.name(), a.value()?);
        }
    }
    if let Some(g) = g.root() {
        for g in g.groups() {
            println!();
            print_group(&g)?;
        }
    }

    Ok(())
}

fn print_group(g: &netcdf::group::Group) -> Result<()> {
    println!("Group: {}", g.name());

    let mut dims = g.dimensions().peekable();
    if dims.peek().is_some() {
        println!("Dimensions:");
        for d in dims {
            if d.is_unlimited() {
                println!("\t{} : Unlimited ({})", d.name(), d.len());
            } else {
                println!("\t{} : ({})", d.name(), d.len());
            }
        }
    }
    let mut types = g.types().peekable();
    if types.peek().is_some() {
        println!("Types:");
        for t in types {
            use netcdf::types::VariableType;
            print!("\t{}: ", t.name());
            match t {
                VariableType::Basic(_) | VariableType::String => unreachable!(),
                VariableType::Opaque(o) => println!("Opaque({})", o.size()),
                VariableType::Enum(_) => println!("Enum"),
                VariableType::Vlen(v) => println!("Vlen({})", v.typ().name()),
                VariableType::Compound(c) => {
                    print!("Compound({{");
                    for field in c.fields() {
                        print!(" {}: {} ", field.name(), field.typ().name());
                    }
                    println!("}})");
                }
            }
        }
    }

    let mut variables = g.variables().peekable();
    if variables.peek().is_some() {
        println!("Variables:");
        for v in variables {
            print!("\t{}", v.name());
            print!("(");
            for d in v.dimensions() {
                print!(" {} ", d.name());
            }
            println!("): {}", v.vartype().name());
            for a in v.attributes() {
                println!("\t\t{} = {:?}", a.name(), a.value()?);
            }
        }
    }
    let mut attributes = g.attributes().peekable();
    if attributes.peek().is_some() {
        println!("Attributes:");
        for a in g.attributes() {
            println!("\t\t{} = {:?}", a.name(), a.value()?);
        }
    }
    for g in g.groups() {
        println!();
        print_group(&g)?;
    }

    Ok(())
}
