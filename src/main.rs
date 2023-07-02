use macroquad::{
    prelude::Color,
    rand::gen_range,
    shapes::draw_rectangle,
    window::{next_frame, Conf}, time::{get_frame_time},
};

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: f32 = 1000.0;
const NUM_NUMBERS: i32 = SCREEN_WIDTH;
fn window_conf() -> Conf {
    Conf {
        window_title: "Sorting Algorithms".to_owned(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT as i32,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let rectangle_width: f32 = (SCREEN_WIDTH / NUM_NUMBERS) as f32;
    let mut numbers: [i32; NUM_NUMBERS as usize] = [0; NUM_NUMBERS as usize];
    let mut colors: [Color; NUM_NUMBERS as usize] = [Color::from_rgba(0, 0, 0, 0);
            NUM_NUMBERS as usize];
    for i in 0..NUM_NUMBERS{
        numbers[i as usize] = gen_range(1, SCREEN_HEIGHT as i32);
        colors[i as usize] = Color::from_rgba(
            gen_range(0, 255),
            gen_range(0, 255),
            gen_range(0, 255), 
            255);
    }
    let mut step_count = 0;
    let mut time_passed = 0.0;
    let mut finished = false;
    let mut printed = false;
    loop {
        for i in 0..NUM_NUMBERS {
            draw_rectangle(
                i as f32 * rectangle_width,
                SCREEN_HEIGHT - numbers[i as usize] as f32,
                rectangle_width,
                numbers[i as usize] as f32,
                colors[i as usize],
            );
        }
        time_passed += get_frame_time();
        if time_passed > 0.2 && !finished{
            let output = optimized_bubble_sort(numbers, colors);
            if output.0 == numbers{
                finished = true;
            } else {
                step_count += 1;
            }
            numbers = output.0;
            colors = output.1;
            time_passed = 0.0;
        }
        if finished && !printed{
            println!("\nSorted in {} steps!", step_count);
            printed = true;
        }
        next_frame().await;
    }
}

fn bubble_sort(mut numbers: [i32; NUM_NUMBERS as usize], mut colors: [Color; NUM_NUMBERS as usize]) -> ([i32; NUM_NUMBERS as usize], [Color; NUM_NUMBERS as usize]){
    for i in 0..NUM_NUMBERS - 1{
        if numbers[i as usize + 1] < numbers[i as usize]{
            let temp_number: i32 = numbers[i as usize];
            let temp_color: Color = colors[i as usize];
            numbers[i as usize] = numbers[i as usize+1];
            colors[i as usize] = colors[i as usize + 1];
            numbers[i as usize + 1] = temp_number;
            colors[i as usize+ 1] = temp_color;
        }
    }
    return (numbers, colors);
}

fn optimized_bubble_sort(mut numbers: [i32; NUM_NUMBERS as usize], mut colors: [Color; NUM_NUMBERS as usize]) -> ([i32; NUM_NUMBERS as usize], [Color; NUM_NUMBERS as usize]){
    for i in 0..NUM_NUMBERS - 1{
        for j in 0..numbers.len() - i as usize - 1 {
            if numbers[j] > numbers[j+1]{
                let temp = numbers[j];
                numbers[j] = numbers[j + 1];
                numbers[j + 1] = temp;
                let temp_color = colors[j];
                colors[j] = colors[j + 1];
                colors[j + 1] = temp_color;
            }
        }
    }
    return (numbers, colors);
}