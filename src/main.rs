use raylib::prelude::*;
use crate::KeyboardKey::*;

const SCREEN_WIDTH: i32 = 960;
const SCREEN_HEIGHT: i32 = 640;

struct Player {
	x: i32,
	y: i32,
	speed: i32,
	health: u32,
	isAttacking: bool,
}

/// An enumerator that stores the "collected" state of the elemental attacks with the idea of the elemental attacks
/// being "charge up" attacks only usable when a meter is filled. The "meter" will likely fill up small amounts based
/// on each enemy defeated, with "charge stones" hidden in places that fill up the meter to the top.
enum ElementState {
	Light,
	Fire,
	Water,
	Wind,
	Earth,
}

/// An enumerator that stores the animation states for the player
enum AnimationState {
	Idle,
	Walk,
	Attack,
}	


fn main() {

	let mut player = Player {
		x: 430,
		y: 320,
		speed: 3,
		health: 3,
		isAttacking: false,
	};
	
	let (mut rl, thread) = raylib::init()
	.size(SCREEN_WIDTH, SCREEN_HEIGHT)
		.title("Yui")
		.build();
	
	// Load player sprite as texture
	let player_tex  = rl.load_texture(&thread, "src/gfx/yui_front.png").unwrap();

	rl.set_target_fps(60);
	while !rl.window_should_close() {

		// Move player with inputs
		if rl.is_key_down(KEY_RIGHT){
			player.x += player.speed;
		}
		if rl.is_key_down(KEY_LEFT){
			player.x -= player.speed;
		}
		if rl.is_key_down(KEY_UP){
			player.y -= player.speed;
		}
		if rl.is_key_down(KEY_DOWN){
			player.y += player.speed;
		}

		
		let mut d = rl.begin_drawing(&thread);
		
		d.clear_background(Color::WHITE);
		d.draw_texture(&player_tex, player.x, player.y, Color::WHITE);
		d.draw_fps(0, 0);
	}
}
