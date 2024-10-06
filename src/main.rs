use std::{io, usize};
use rand::Rng;

const WIDTH:usize = 10;
const HEIGHT:usize = 10;


fn main() {

    let mut cells_number:u32 = get_initial_cells();

    let mut population = [[0 as u8; WIDTH]; HEIGHT] ;

    while cells_number > 0 {

        let y = rand::thread_rng().gen_range(0..WIDTH);
        let x = rand::thread_rng().gen_range(0..HEIGHT);

        if population[x][y] == 0 {
            population[x][y] = 1;
            cells_number -= 1;
        }
    }

    while not_all_dead(&population) {

        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");
        println!("Siguiente generacion:"); 
        print_population(&population);

        population = get_next_generation(&population);  

        

        // Add a small delay (optional)
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    

}



fn get_initial_cells() -> u32{

    let cells_number:u32 = loop{

        println!("Introduce la cantidad de células con las que quieres comenzar: ");
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input <= (WIDTH * HEIGHT) as u32 {
            break input;
        } else {
            println!("El numero de celulas debe ser menor que el tamaño de la poblacion ({})", WIDTH * HEIGHT);
        }

        
    };

    return cells_number;
}


fn print_population(population: &[[u8; WIDTH]; HEIGHT]){
    for row in population {
        for &cell in row.iter() {
            print!("{} ", if cell == 1 { 'O' } else { ' ' });
        }
        println!();
    
    }
}

fn get_next_generation(population: &[[u8; WIDTH]; HEIGHT]) -> [[u8; WIDTH]; HEIGHT]{
    let mut new_population = [[0 as u8; WIDTH]; HEIGHT];


    let checks = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let living_neighbors = check_living_neighbors(&population, row, col, &checks);

            if population[row][col] == 0 {
                if living_neighbors == 3 {
                    new_population[row][col] = 1;
                } else {
                    new_population[row][col] = 0;
                }
            } else {
                if living_neighbors < 2 || living_neighbors > 3 {
                    new_population[row][col] = 0;
                } else {
                    new_population[row][col] = 1;
                }
            }

            

        }
    }

    return new_population;
}

fn check_living_neighbors(population: &[[u8; WIDTH]; HEIGHT], row: usize, col: usize, checks: &[(i32, i32)]) -> u8{
    let mut living_neighbors = 0;

    for (dx, dy) in checks {
        let neighbor_row = row as i32 + dx;
        let neighbor_col = col as i32 + dy;

        // Check if the neighbor's coordinates are within the grid bounds
        if neighbor_row >= 0 && neighbor_row < HEIGHT as i32 && neighbor_col >= 0 && neighbor_col < WIDTH as i32 {
            // Convert back to usize safely after bounds check
            let neighbor_row = neighbor_row as usize;
            let neighbor_col = neighbor_col as usize;

            if population[neighbor_row][neighbor_col] == 1 {
                living_neighbors += 1;
            }
        } 
    }

    return living_neighbors;
}

fn not_all_dead(population: &[[u8; WIDTH]; HEIGHT]) -> bool{
    for row in population {
        for &cell in row.iter() {
            if cell == 1 {
                return true;
            }
        }
    }
    return false;
}

