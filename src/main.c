#include "raylib.h"

#define SCREEN_WIDTH 240
#define SCREEN_HEIGHT 160
#define TITLE "Yui"
#define FPS 60
#define dt GetFrameTime()
//root directory path(because for some reason, raylib's stupid build script doesn't
//recognize the root directory, so for file loading, the full path is needed. this
//reduces the path on screen
#define ROOT_DIR "/home/bella/Documents/dev/games/yui_proto"

//boolean values
#define true 1
#define false 0

//struct Player {
	//Vector2 position;
	//float speed;
	//float cameraSpeed;
	//int health;
	//int isAttacking;
	
//};

int main(void){
	//struct Player player;
	
	InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, TITLE);

	// NOTE: Textures MUST be loaded after window initialization (OpenGL context is required)
	Texture2D playerSprite = LoadTexture(ROOT_DIR"/gfx/yui_front.png");
	Texture2D test_map = LoadTexture(ROOT_DIR"/gfx/test_map.png");

	Vector2 playerPosition = {(float)SCREEN_WIDTH/2, (float)SCREEN_HEIGHT/2 };

	//set up basic camera
	Camera2D camera = {0};
	camera.target = (Vector2){ playerPosition.x, playerPosition.y};
	camera.offset=(Vector2){ SCREEN_WIDTH/2.0f, SCREEN_HEIGHT/2.0f};
	camera.rotation = 0.0f;
	camera.zoom = 1.0f;

	SetTargetFPS(FPS);
	//main game loop
	while (!WindowShouldClose()){
		//update variables here
		if (IsKeyDown(KEY_RIGHT)){
			playerPosition.x += 2.0f;
		}
		if (IsKeyDown(KEY_LEFT)){
			playerPosition.x -= 2.0f;
		}
		if (IsKeyDown(KEY_UP)){
			playerPosition.y -= 2.0f;
		}
		if (IsKeyDown(KEY_DOWN)){
			playerPosition.y += 2.0f;
		}

		//Camera "targets" the player
		camera.target = (Vector2){ playerPosition.x, playerPosition.y};

		//Draw
		BeginDrawing();

			ClearBackground(BLACK);

			BeginMode2D(camera);
				//DrawText("Hello, YUI", 0, 0, 20, WHITE);
				DrawTexture(test_map, 0, 0, WHITE);
				DrawTexture(playerSprite, playerPosition.x - playerSprite.width/2, 
				playerPosition.y - playerSprite.height/2, WHITE);
		EndDrawing();
		EndMode2D();
	}

	//De-initialization
	UnloadTexture(test_map);
	UnloadTexture(playerSprite);
	CloseWindow();

	return 0;
}
