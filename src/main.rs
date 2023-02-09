use raylib::prelude::*;
use crate::KeyboardKey::*;
use tiled::Loader;

const SCREEN_WIDTH: i32 = 960;
const SCREEN_HEIGHT: i32 = 640;

struct Player {
	position: Vector2,
	speed: f32,
	health: u32,
	isAttacking: bool,
	cameraSpeed: f32,
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

/// An enumerator that stores the animation states for the player, as well as the direction
/// of the player(coming soon)
enum AnimationState {
	Idle,
	Walk,
	Attack,
}	


fn main() {

	// load tilemaps and tilesets
	let mut loader = Loader::new();
	let testMap = loader.load_tmx_map("src/gfx/maps/test.tmx").unwrap();
	let testmapTileset = loader.load_tsx_tileset("src/gfx/maps/overworld_tiles.tsx").unwrap();
	let mut player = Player {
		// x: 430.0,
		// y: 320.0,
		position: Vector2::new(430.0, 320.0),
		speed: 150.0,
		health: 3,
		isAttacking: false,
		cameraSpeed: 2.45,
	};

	//set up camera
	let mut camera = Camera2D {
		target: Vector2::new(player.position.x, player.position.y),
		offset: Vector2::new(0.0, 0.0),
		rotation: 0.0,
		zoom: 1.0,
	};
	
	let (mut rl, thread) = raylib::init()
		.size(SCREEN_WIDTH, SCREEN_HEIGHT)
		.title("Yui")
		.build();
	
	// Load player sprite as texture
	let player_tex = rl.load_texture(&thread, "src/gfx/yui_front.png").unwrap();
	let testmap_tex = rl.load_texture(&thread, "src/gfx/map.png").unwrap();

	rl.set_target_fps(60);
	while !rl.window_should_close() {
		// Move player with inputs
		if rl.is_key_down(KEY_RIGHT){
			player.position.x += player.speed * rl.get_frame_time();
			camera.offset.x -= player.cameraSpeed;
		}
		if rl.is_key_down(KEY_LEFT){
			player.position.x -= player.speed * rl.get_frame_time();
			camera.offset.x += player.cameraSpeed;		}
		if rl.is_key_down(KEY_UP){
			player.position.y -= player.speed * rl.get_frame_time();
			camera.offset.y += player.cameraSpeed;
		}
		if rl.is_key_down(KEY_DOWN){
			player.position.y += player.speed * rl.get_frame_time();
			camera.offset.y -= player.cameraSpeed;
		}

		camera.target = Vector2::new(player.position.x * rl.get_frame_time(), player.position.y * rl.get_frame_time());

		
		let mut d = rl.begin_drawing(&thread);
		d.clear_background(Color::WHITE);
		let mut d2 = d.begin_mode2D(camera);
		d2.draw_texture(&testmap_tex, 0, 0, Color::WHITE);
		d2.draw_texture(&player_tex, player.position.x as i32, player.position.y as i32, Color::WHITE);
		d2.draw_fps(0, 0);
	}
}
