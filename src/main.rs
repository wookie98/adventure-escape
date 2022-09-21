use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use std::{thread, time::Duration, fmt::format};
use components::{
	Enemy,  FromEnemy, FromPlayer, Bullet, Movable,
	Player, SpriteSize, Velocity, Lives, AmmoDrop, Ammo, TopText
};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use std::collections::HashSet;
use ammoDrop::AmmoDropsPlugin;
mod components;
mod player;
mod enemy;
mod ammoDrop;

const BOTTOM:f32=-238.0;
const MIDDLE:f32=-012.0;
const ABOVE:f32=212.0;

const PLAYER_SPRITE: &str = "run__007.png";
const PLAYER_SIZE: (f32, f32) = (150., 75.);

const NINJA_SPRITE: &str = "ninja.png";
const NINJA_SIZE: (f32, f32) = (150., 75.);

const SAMURAI_SPRITE: &str = "samurai.png";
const SAMURAI_SIZE: (f32, f32) = (150., 75.);

const AMMO_SPRITE: &str = "gun.png";
const AMMO_SIZE: (f32, f32) = (150., 75.);


const PLAYER_BULLET_SPRITE: &str = "bullet.png";
const PLAYER_BULLET_SIZE: (f32, f32) = (9., 54.);

//const ENEMY_SPRITE: &str = "samurai.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);

const ENEMY_BULLET_SIZE: (f32, f32) = (17., 55.);

//const SAM_SHEET: &str = "samurai.png";
//const SAM_LEN: usize = 60;

const SPRITE_SCALE: f32 = 1.5;


const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

const PLAYER_RESPAWN_DELAY: f64 = 2.;
const ENEMY_MAX: u32 = 9;

// region:    --- Resources
pub struct WinSize {
	pub w: f32,
	pub h: f32,
}


struct GameTextures {
	player: Handle<Image>,
	player_bullet: Handle<Image>,
	samurai: Handle<Image>,
	ninja: Handle<Image>,
	ammo: Handle<Image>,
//player_bullet: Handle<_>,
//	enemy_bullet: Handle<Image>,

}

struct PlayerAlive {
	on: bool,       // alive
	last_hit: f64, // -1 if not shot
}
impl Default for PlayerAlive {
	fn default() -> Self {
		Self {
			on: false,
			last_hit: -1.,
		}
	}
}

impl PlayerAlive {
	pub fn hit(&mut self, timeSinceStart: f64) {
		self.on = false;
		self.last_hit = timeSinceStart;
		

	}
	pub fn spawned(&mut self) {
		self.on = true;
		self.last_hit = -1.;
	}
}
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}
struct EnemyCount(u32);
struct MapSpeed(f32);
struct Score(f32);

struct ScoreInc(f32);


struct LastDeath(f64);
struct Chance(i32);
struct PlayerLife(i32);

fn main() {
    //println!("Hello, world!");
    App::new()
		.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
		.insert_resource(WindowDescriptor {
			title: "escape adventure".to_string(),
			width: 598.0,
			height: 676.0,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(PlayerPlugin)
		.add_plugin(EnemyPlugin)
		.add_plugin(AmmoDropsPlugin)
		.add_plugin(OverlayPlugin { font_size: 32.0, ..default() })
		.add_startup_system(init)
		.add_startup_system(setup_system)
		 .add_system_set(
			SystemSet::on_enter(GameState::Playing)
			.with_system(setup_system)
			
		)
		.add_system_set(
			SystemSet::on_update(GameState::Playing)
			
		.with_system(movable_system)
		.with_system(player_bullet_hit_enemy_system)
		.with_system(enemy_hit_player_system)		
		.with_system(ammo_hit_player_system)
		.with_system(text_update_system)
		)
		.add_system_set(
			SystemSet::on_enter(GameState::GameOver)
			.with_system(game_over)
			

		)
		.add_system_set(
			SystemSet::on_update(GameState::GameOver)
			.with_system(gameover_keyboard)
		)
		.add_system_set(
			SystemSet::on_exit(GameState::Playing)
			.with_system(teardown)
		)
		.add_system_set(
			SystemSet::on_exit(GameState::GameOver)
			.with_system(teardown)
		)
		.add_state(GameState::Playing)
		.run();


}

fn teardown(mut commands: Commands, entities: Query<Entity, (Without<Camera>)>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn init(mut commands: Commands,)
{
	commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut windows: ResMut<Windows>,
	mut player_alive: ResMut<PlayerAlive>,
	
) {
	// capture window size
	let window = windows.get_primary_mut().unwrap();
	let (win_w, win_h) = (window.width(), window.height());

	// position window (for tutorial)
	// window.set_position(IVec2::new(2780, 4900));

	// add WinSize resource
	let win_size = WinSize { w: win_w, h: win_h };
	commands.insert_resource(win_size);

	
	// add GameTextures resource
	let game_textures = GameTextures {
		player: asset_server.load(PLAYER_SPRITE),
		samurai: asset_server.load(SAMURAI_SPRITE),
		ninja: asset_server.load(NINJA_SPRITE),
    	ammo: asset_server.load(AMMO_SPRITE),
		player_bullet: asset_server.load(PLAYER_BULLET_SPRITE),
	};

	commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Score:",
                TextStyle {
                    font: asset_server.load("Heebo-VariableFont_wght.ttf"),
                    font_size: 70.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(TopText);
		
		let background_image: Handle<Image> = asset_server.load("background.png");
		
		commands	
			.spawn_bundle(SpriteBundle {
				texture: background_image.into(),
				transform: Transform::from_scale(Vec3::new(1., 1., 0.0)),
				..Default::default()
			});
		
	player_alive.on=false;
	commands.insert_resource(game_textures);
	commands.insert_resource(EnemyCount(0));
	commands.insert_resource(MapSpeed(-0.1));
	commands.insert_resource(Score(0.));
	commands.insert_resource(ScoreInc(0.));
	commands.insert_resource(Chance(15));
	commands.insert_resource(LastDeath(0.));
	commands.insert_resource(PlayerLife(3));
	
}

fn movable_system(
	mut commands: Commands,
	win_size: Res<WinSize>,
	mut enemy_count: ResMut<EnemyCount>,
	mut map_speed: ResMut<MapSpeed>,
	mut score: ResMut<Score>,
	mut chance: ResMut<Chance>,
	mut scoreInc: ResMut<ScoreInc>,
	mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
	score.0+=50.*TIME_STEP;
		scoreInc.0+=50.*TIME_STEP;
		if scoreInc.0>=1000.0 && score.0<30000.
		{
			map_speed.0-=0.1;
			chance.0+=1;
			scoreInc.0=0.;

		}
	for (entity, velocity, mut transform, movable) in query.iter_mut() {
		let translation = &mut transform.translation;
		if movable.enemy || movable.ammo
		{
			translation.x += (velocity.x+map_speed.0) * TIME_STEP * BASE_SPEED;
		}
		else
		{
			translation.x += velocity.x * TIME_STEP * BASE_SPEED;
			translation.y += velocity.y * TIME_STEP * BASE_SPEED;

		}
		
		

		
		if movable.auto_despawn {
			// despawn when out of screen
			const MARGIN: f32 = 200.;
			if translation.y > win_size.h / 2. + MARGIN
				|| translation.y < -win_size.h / 2. - MARGIN
				|| translation.x > win_size.w / 2. + MARGIN
				|| translation.x < -win_size.w / 2. - MARGIN
			{
				if movable.enemy
				{
					enemy_count.0-=1;
				}
				commands.entity(entity).despawn();
			}
		}
	}
}

fn player_bullet_hit_enemy_system(
	mut commands: Commands,
	mut enemy_count: ResMut<EnemyCount>,
	mut map_speed: ResMut<MapSpeed>,
	mut chance: ResMut<Chance>,
	mut score: ResMut<Score>,
	mut scoreInc: ResMut<ScoreInc>,
	bullet_query: Query<(Entity, &Transform, &SpriteSize), (With<Bullet>, With<FromPlayer>)>,
	mut enemy_query: Query<(Entity, &Transform, &SpriteSize, &mut Lives), With<Enemy>>,
) {
	let mut despawned_entities: HashSet<Entity> = HashSet::new();

	// iterate through the shots
	for (bullet_entity, bullet_tf, bullet_size) in bullet_query.iter() {
		if despawned_entities.contains(&bullet_entity) {
			continue;
		}

		let bullet_scale = Vec2::from(bullet_tf.scale.xy());

		// iterate through the enemies
		for (enemy_entity, enemy_tf, enemy_size,mut enemy_lives) in enemy_query.iter_mut() {
			if despawned_entities.contains(&enemy_entity)
				|| despawned_entities.contains(&bullet_entity)
			{
				continue;
			}

			let enemy_scale = Vec2::from(enemy_tf.scale.xy());

			// determine if collision
			let collision = collide(
				bullet_tf.translation,
				bullet_size.0 * bullet_scale/8.,
				enemy_tf.translation,
				enemy_size.0/4. * enemy_scale,
			);

			// perform collision
			if let Some(_) = collision {
				// remove the enemy
				//println!("pre{}", enemy_lives.lives);
				enemy_lives.lives-=1;
				//println!("post{}", enemy_lives.lives);
				if enemy_lives.lives<=0
				{
				commands.entity(enemy_entity).despawn();
				despawned_entities.insert(enemy_entity);
				enemy_count.0 -= 1;
				}


				commands.entity(bullet_entity).despawn();
				despawned_entities.insert(bullet_entity);

				score.0+=200.;
				scoreInc.0+=200.;
				if scoreInc.0>=1000. && score.0<30000.
				{
					map_speed.0-=0.1;
					chance.0+=1;
					scoreInc.0=0.;

				}

			}
		}
	}
}
fn enemy_hit_player_system(
	mut commands: Commands,
	mut player_alive: ResMut<PlayerAlive>,
	time: Res<Time>,
	mut lastDeath: ResMut<LastDeath>,
	mut playerLife: ResMut<PlayerLife>,
	//mut over: ResMut<GameStateOver>,
	mut state: ResMut<State<GameState>>,
	enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
	player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
) {
	if let Ok((player_entity, player_tf, player_size)) = player_query.get_single() {
		let player_scale = Vec2::from(player_tf.scale.xy());
		
		for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
			let enemy_scale = Vec2::from(enemy_tf.scale.xy());

			// determine if collision
			let collision = collide(
				enemy_tf.translation,
				enemy_size.0/4. * enemy_scale,
				player_tf.translation,
				player_size.0/4. * player_scale,
			);

			// perform the collision
			if let Some(_) = collision {
				// remove the player
				if time.seconds_since_startup()>lastDeath.0+5.0
				{
				commands.entity(player_entity).despawn();
				lastDeath.0=time.seconds_since_startup();
				player_alive.hit(time.seconds_since_startup());
				playerLife.0-=1;
				if(playerLife.0<=0)
				{
					state.set(GameState::GameOver).unwrap();
					
				    thread::sleep(Duration::from_millis(1000));
					

				}
				// remove
				commands.entity(enemy_entity).despawn();


				break;
				}
			}
		}
	}
}

fn ammo_hit_player_system(
	mut commands: Commands,	
	ammo_query: Query<(Entity, &Transform, &SpriteSize), With<AmmoDrop>>,
	mut player_query: Query<(Entity, &Transform, &SpriteSize, &mut Ammo), With<Player>>,
) {
	
	if let Ok((player_entity, player_tf, player_size,mut player_ammo)) = player_query.get_single_mut() {
		let player_scale = Vec2::from(player_tf.scale.xy());
		
		for (ammo_entity, ammo_tf, ammo_size) in ammo_query.iter() {
			let ammo_scale = Vec2::from(ammo_tf.scale.xy());
		
			// determine if collision
			let collision = collide(
				ammo_tf.translation,
				ammo_size.0/4. * ammo_scale,
				player_tf.translation,
				player_size.0/4. * player_scale,
			);

			// perform the collision
			if let Some(_) = collision {

				
				commands.entity(ammo_entity).despawn();
				player_ammo.ammo+=5;

				break;
				}
			}
		}
	}


	fn text_update_system(
		mut score: ResMut<Score>,
		mut playerLife: ResMut<PlayerLife>,
		mut query: Query<&mut Text, With<TopText>>,
		player_query: Query<&mut Ammo, With<Player>>
	) {
		for mut text in &mut query {
			if let Ok(player_ammo) = player_query.get_single() { 
				text.sections[0].value = format!("score: {} lives: {} ammo: {}",score.0.floor(),playerLife.0,player_ammo.ammo);	
			}
		}
	}
	fn gameover_keyboard(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
		if keyboard_input.just_pressed(KeyCode::Space) {
			state.set(GameState::Playing).unwrap();
		}
	}
	fn game_over(
		mut commands: Commands,
		asset_server: Res<AssetServer>,
		score: ResMut<Score>
	){
	commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                format!("Game Over, Score:{}\n Press Star to play again",score.0),
                TextStyle {
                    font: asset_server.load("Heebo-VariableFont_wght.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },

				

                ..default()
            }),
        )
        .insert(TopText);
	}