use macroquad::{
    prelude::Color,
    rand::gen_range,
    shapes::draw_rectangle,
    window::{next_frame, Conf}, time::{get_frame_time},
};

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: f32 = 1000.0;
const NUM_NUMBERS: i32 = 100;
const TIME_DELAY: f32 = 0.00000000000000;
const RECTANGLE_WIDTH: f32 = (SCREEN_WIDTH / NUM_NUMBERS) as f32;
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
    for _ in 0..100{gen_range(0, 10);}
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
    let mut time_passed = TIME_DELAY;
    let mut swaps = -1;
    let mut index_1 = 0;
    let mut index_2 = 0;
    let mut last_element = 0;
    let mut sorted = false;
    let mut printed = false;
    let mut total_comparisons = 0;
    let mut sorts_in_a_row = 0;
    loop {
        for i in 0..NUM_NUMBERS {
            draw_rectangle(
                i as f32 * RECTANGLE_WIDTH,
                SCREEN_HEIGHT - numbers[i as usize] as f32,
                RECTANGLE_WIDTH,
                numbers[i as usize] as f32,
                colors[i as usize],
            );
        }
        time_passed += get_frame_time();
        if time_passed >= TIME_DELAY && !sorted{
            // (numbers, colors, index_1, index_2, swaps, total_comparisons, sorts_in_a_row) = bubble_sort(numbers, colors, index_1, index_2, swaps, total_comparisons, sorts_in_a_row);
            (numbers, colors, index_1, index_2, swaps, last_element, total_comparisons) = optimized_bubble_sort(numbers, colors, index_1, index_2, swaps, last_element, total_comparisons);
            if last_element == NUM_NUMBERS - 1 || sorts_in_a_row == NUM_NUMBERS - 1{
                sorted = true;
            }
            time_passed = 0.0;
        }
        if !sorted{
            draw_rectangle(index_1 as f32 * RECTANGLE_WIDTH, 0.0, RECTANGLE_WIDTH, SCREEN_HEIGHT, Color::from_rgba(180, 180, 180, 180));
            draw_rectangle(index_2 as f32 * RECTANGLE_WIDTH, 0.0, RECTANGLE_WIDTH, SCREEN_HEIGHT, Color::from_rgba(180, 180, 180, 180));
        } else if sorted && !printed{
            printed = true;
            println!("\nSorted in {} swaps!", swaps);
            println!("Total Comparisons: {}", total_comparisons);
        }
        next_frame().await;
    }
}

fn bubble_sort(mut numbers: [i32; NUM_NUMBERS as usize], mut colors: [Color; NUM_NUMBERS as usize], mut index_1: usize, mut index_2: usize, mut swaps: i32, mut total_comparisons: i32, mut sorts_in_a_row: i32) -> ([i32; NUM_NUMBERS as usize], [Color; NUM_NUMBERS as usize], usize, usize, i32, i32, i32){
    let did_a_sort;
    if swaps == -1{
        return (numbers, colors, 0, 1, 0, 0, 0)
    }
    if index_2 == numbers.len() - 1{
        (numbers, colors, swaps, total_comparisons, did_a_sort) = swap(numbers, colors, index_1, index_2, swaps, total_comparisons);
        index_1 = 0;
        index_2 = 1;
        sorts_in_a_row = if did_a_sort {0} else {sorts_in_a_row + 1};
        return (numbers, colors, index_1, index_2, swaps, total_comparisons, sorts_in_a_row)
    }
    (numbers, colors, swaps, total_comparisons, did_a_sort) = swap(numbers, colors, index_1, index_2, swaps, total_comparisons);
    sorts_in_a_row = if did_a_sort {0} else {sorts_in_a_row + 1};
    (numbers, colors, index_1 + 1, index_2 + 1, swaps, total_comparisons, sorts_in_a_row)
}

fn optimized_bubble_sort(mut numbers: [i32; NUM_NUMBERS as usize], mut colors: [Color; NUM_NUMBERS as usize], mut index_1: usize, mut index_2: usize, mut swaps: i32, last_element: i32, mut total_comparisons: i32) -> ([i32; NUM_NUMBERS as usize], [Color; NUM_NUMBERS as usize], usize, usize, i32, i32, i32){
    if swaps == -1{
        return (numbers, colors, 0, 1, 0, 0, 0)
    }
    if index_2 == numbers.len() - 1 - last_element as usize{
        (numbers, colors, swaps, total_comparisons, _) = swap(numbers, colors, index_1, index_2, swaps, total_comparisons);
        index_1 = 0;
        index_2 = 1;
        return (numbers, colors, index_1, index_2, swaps, last_element + 1, total_comparisons)
    }
    (numbers, colors, swaps, total_comparisons, _) = swap(numbers, colors, index_1, index_2, swaps, total_comparisons);
    (numbers, colors, index_1 + 1, index_2 + 1, swaps, last_element, total_comparisons)
}

fn swap(mut numbers: [i32; NUM_NUMBERS as usize], mut colors: [Color; NUM_NUMBERS as usize], index_1: usize, index_2: usize, mut swaps: i32, total_comparisons: i32) -> ([i32; NUM_NUMBERS as usize], [Color; NUM_NUMBERS as usize], i32, i32, bool){
    if numbers[index_1] > numbers[index_2]{
        let temp_number = numbers[index_1];
        let temp_color = colors[index_1];
        colors[index_1] = colors[index_2];
        colors[index_2] = temp_color;
        numbers[index_1] = numbers[index_2];
        numbers[index_2] = temp_number;
        swaps += 1;
        return (numbers, colors, swaps, total_comparisons + 1, true)
    }
    (numbers, colors, swaps, total_comparisons + 1, false)
}