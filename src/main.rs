use std::io;


#[derive(Debug)]
struct Vector2D {
    x: f64,
    y: f64,
    magnitud: f64,
    angle: f64
}


impl Vector2D {

    fn build_with_coords(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            magnitud: calc_magnitud(x, y),
            angle: calc_angle(x, y)
        }
    }

    fn build_with_magnitud(magnitud: f64, angle: f64) -> Self {
        let (x, y) = decompose(magnitud, angle);
        Self {
            x,
            y,
            magnitud,
            angle
        }
    }

    fn sum(&self, other: &Vector2D) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Self {
            x,
            y,
            magnitud: calc_magnitud(x, y),
            angle: calc_angle(x, y),
        }
    }

    fn diff(&self, other: &Vector2D) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        Self {
            x,
            y,
            magnitud: calc_magnitud(x, y),
            angle: calc_angle(x, y),
        }
    }
    


}

fn decompose(magnitud: f64, angle: f64) -> (f64, f64) {
    (magnitud * angle.to_radians().cos(), magnitud * angle.to_radians().sin())
}

fn calc_magnitud(x: f64, y: f64) -> f64 {
    (x.powf(2.0) + y.powf(2.0)).sqrt()
}

fn calc_angle(x: f64, y: f64) -> f64 {
    (y / x).atan().to_degrees()
}

fn calc_point_product(vect1: &Vector2D, vect2: &Vector2D) -> f64 {
    (vect1.x * vect2.x) + (vect1.y * vect2.y)
}

fn main() {
    
    println!("Bienvenido a la calculadora vectorial, escoge que función quieres realizar");

    loop {
        
        
        println!("1- Suma vectorial.");
        println!("2- Resta vectorial.");
        println!("3- Descomposición vectorial.");
        println!("4- Calcular ángulo.");
        println!("5- Calcular magnitud.");
        println!("6- Producto punto.");
        println!("7- Salir");


        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Error en la lectura");

        let choice: Result<i32, _> = choice.trim().parse();

        match choice {

            Ok(n) => match n {
                1 => vectorial_sum(),
                2 => vectorial_diff(),
                3 => vectorial_decompose(),
                4 => angle_calc(),
                5 => magnitud_calc(),
                6 => point_product(),
                7 => break,
                _ => println!("Opción incorrecta"),
            },
            Err(_) => println!("Opción incorrecta"),
        }
    }
}

fn input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error en la lectura");

        match input.trim().parse() {
            Ok(n) => return n,
            Err(_) => println!("Por favor, ingrese un número válido.\n"),
        }
    }
}

fn input_i(prompt: &str) -> i64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error en la lectura");

        match input.trim().parse() {
            Ok(n) => return n,
            Err(_) => println!("Por favor, ingrese un número válido.\n"),
        }
    }
}

fn vectorial_sum() {

    let nu_vects = input_i(&format!("Escriba la cantidad de vectores a sumar "));

    let mut vect_resultante = Vector2D::build_with_coords(0.0,0.0);

    for n in 1..=nu_vects {

        let x_n = input(&format!("Escriba la coordenada X del vector {}: ", n));
        let y_n = input(&format!("Escriba la coordenada Y del vector {}: ", n));

        let vector = Vector2D::build_with_coords(x_n, y_n);
        vect_resultante = vect_resultante.sum(&vector);

        if n != nu_vects { println!("\nEl vector resultante actual es {:?}\n", vect_resultante); }

    }

    println!("\nEl vector resultante final es {:?}\n", vect_resultante);

}

fn vectorial_diff() {
    let nu_vects = input_i(&format!("Escriba la cantidad de vectores a restar "));

    let mut vect_resultante = Vector2D::build_with_coords(0.0,0.0);

    for n in 1..=nu_vects {

        let x_n = input(&format!("Escriba la coordenada X del vector {}: ", n));
        let y_n = input(&format!("Escriba la coordenada Y del vector {}: ", n));

        let vector = Vector2D::build_with_coords(x_n, y_n);
        vect_resultante = vect_resultante.diff(&vector);

        if n != nu_vects { println!("\nEl vector resultante actual es {:?}\n", vect_resultante); }

    }

    println!("\nEl vector resultante final es {:?}\n", vect_resultante);

}

fn vectorial_decompose() {

    let magnitud = input(&format!("Escriba la magnitud del vector"));
    let angle = input(&format!("Escriba el angulo del vector"));

    let vect = Vector2D::build_with_magnitud(magnitud, angle);

    println!("\nEl vector es {:?}\n", vect);
    
}

fn angle_calc() {
    let x = input(&format!("Escriba la coordenada X del vector: "));
    let y = input(&format!("Escriba la coordenada Y del vector: "));

    let vect = Vector2D::build_with_coords(x,y);

    println!("\nEl angulo es {:?}°\n", vect.angle);

}

fn magnitud_calc() {
    let x = input(&format!("Escriba la coordenada X del vector: "));
    let y = input(&format!("Escriba la coordenada Y del vector: "));

    let vect = Vector2D::build_with_coords(x,y);

    println!("\nLa magnitud es {:?}\n", vect.magnitud);
    
}

fn point_product() {
    let x_1 = input(&format!("Escriba la coordenada X del vector 1: "));
    let y_1 = input(&format!("Escriba la coordenada Y del vector 1: "));

    let x_2 = input(&format!("Escriba la coordenada X del vector 2: "));
    let y_2 = input(&format!("Escriba la coordenada Y del vector 2: "));

    let vect1 = Vector2D::build_with_coords(x_1, y_1);
    let vect2 = Vector2D::build_with_coords(x_2, y_2);

    let point_product = calc_point_product(&vect1, &vect2);

    println!("\nEl producto punto es {:?}\n", point_product);
}
