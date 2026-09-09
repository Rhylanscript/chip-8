use chip8::display::Display;

#[test]
fn drawing_sprite_on_blank_screen_sets_pixels_no_collision() {
    let mut display = Display::new();

    let sprite = [0xF0u8];

    let collision = display.draw_sprite(0, 0, &sprite);

    assert_eq!(collision, false);

    assert_eq!(display.pixels[0], true);
    assert_eq!(display.pixels[1], true);
    assert_eq!(display.pixels[2], true);
    assert_eq!(display.pixels[3], true);
    assert_eq!(display.pixels[4], false);
}

#[test]
fn drawing_same_sprite_twice_erases_it_and_reports_collision() {
    let mut display = Display::new();
    let sprite = [0xF0u8];

    display.draw_sprite(0, 0, &sprite);
    let collision = display.draw_sprite(0, 0, &sprite);

    assert_eq!(collision, true);

    assert_eq!(display.pixels[0], false);
    assert_eq!(display.pixels[1], false);
}

#[test]
fn clear_resets_all_pixels_to_off() {
    let mut display = Display::new();
    let sprite = [0xFFu8];

    display.draw_sprite(0, 0, &sprite);
    display.clear();

    assert!(display.pixels.iter().all(|&pixel| pixel == false))
}
