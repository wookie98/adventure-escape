use crate::components::{FromPlayer, Bullet, Movable, Player, SpriteSize, Velocity, Ammo, PlayerState, PLAYER_STATE, LEVEL,PlayerLevel};
use crate::{
	GameTextures, PlayerAlive, WinSize, PLAYER_BULLET_SIZE, PLAYER_RESPAWN_DELAY, PLAYER_SIZE,
	SPRITE_SCALE,BOTTOM,MIDDLE,ABOVE
};
use bevy::prelude::*;
use bevy::time::FixedTimestep;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(PlayerAlive::default())
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(0.5))
					.with_system(player_spawn_system),
			)
			
			.add_system(player_keyboard_event_system)
			.add_system(player_fire_system)
			.add_system(player_movement_system);
	}
}

fn player_spawn_system(
	mut commands: Commands,
	mut player_alive: ResMut<PlayerAlive>,
	time: Res<Time>,
	game_textures: Res<GameTextures>,
	win_size: Res<WinSize>,
) {
	let now = time.seconds_since_startup();
	let last_hit = player_alive.last_hit;

	if !player_alive.on && (last_hit == -1. || now > last_hit + PLAYER_RESPAWN_DELAY) {
		// add player
		//let bottom = -win_size.h / 2.;
		commands
			.spawn_bundle(SpriteBundle {
				texture: game_textures.player.clone(),
				transform: Transform {
					translation: Vec3::new(
						-280.,
						MIDDLE + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.,
						10.,
					),
					scale: Vec3::new(SPRITE_SCALE/11.0, SPRITE_SCALE/11., 1.),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Player)
			.insert(SpriteSize::from(PLAYER_SIZE))
			//.insert(Movable { auto_despawn: false })
			.insert(Velocity { x: 0., y: 0. })
			.insert(PlayerState{state:PLAYER_STATE::Run})
			.insert(PlayerLevel{level:LEVEL::Middle})
            .insert(Ammo{ammo:5});

		player_alive.spawned();
	}
}

fn player_fire_system(
	mut commands: Commands,
	kb: Res<Input<KeyCode>>,
	game_textures: Res<GameTextures>,
	mut query: Query<(&Transform, &mut Ammo), With<Player>>,
) {
	if let Ok(mut player_tf) = query.get_single_mut() {
		if kb.just_pressed(KeyCode::Space)&& player_tf.1.ammo>0  {
			
			let (x, y) = (player_tf.0.translation.x,player_tf.0.translation.y);
			let x_offset = 25.;
			player_tf.1.ammo=player_tf.1.ammo-1;
			
			let mut spawn_bullet = |x_offset: f32| {
				    
				commands
					.spawn_bundle(SpriteBundle {
						//texture: game_textures.player_bullet.clone(),
						transform: Transform {
							translation: Vec3::new(x + x_offset, y + 15., 1.),
							scale: Vec3::new(SPRITE_SCALE*1.5, SPRITE_SCALE*1.5, 1.),
							..Default::default()
						},
						..Default::default()
					})
					.insert(Bullet)
					.insert(FromPlayer)
					.insert(SpriteSize::from(PLAYER_BULLET_SIZE))
					.insert(Movable { auto_despawn: true, enemy:false, ammo:false })
					.insert(Velocity { x: 1., y: 0. });
					//.insert(Ammo { ammo:updated_ammo });
			};

			spawn_bullet(x_offset);
			
		}
	}
}

fn player_keyboard_event_system(
	kb: Res<Input<KeyCode>>,
	mut query: Query<(&mut PlayerState, &PlayerLevel, &mut Velocity), With<Player>>,
) {
	if let Ok(mut State) = query.get_single_mut() {
		if kb.pressed(KeyCode::Up) && !(State.1.level==LEVEL::Above) && State.0.state==PLAYER_STATE::Run {
			State.0.state = PLAYER_STATE::JumpUp;
			State.2.y=5.0;
		} 
		else if kb.pressed(KeyCode::Down) && !(State.1.level==LEVEL::Below) && State.0.state==PLAYER_STATE::Run {
			State.0.state = PLAYER_STATE::JumpDown;
			State.2.y=-2.0;
		} 
		
	}
}

fn player_movement_system(mut query: Query<(&mut Transform,&mut PlayerState, &mut PlayerLevel,&mut Velocity ), With<Player>>) {
	for mut transform in query.iter_mut() {
		let translation = &mut transform.0.translation;
		let state=&mut transform.1.state;
		let level=&mut transform.2.level;
		let vel=&mut transform.3.y;
		if *state==PLAYER_STATE::JumpUp && *level==LEVEL::Below && translation.y<MIDDLE+ PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.
		{
			translation.y+=*vel;
		}
		if *state==PLAYER_STATE::JumpUp && *level==LEVEL::Middle && translation.y<ABOVE+ PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.
		{
			translation.y+=*vel;
		}
		if *state==PLAYER_STATE::JumpDown && *level==LEVEL::Above && translation.y>MIDDLE+ PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.
		{
			translation.y+=*vel;
		}
		if *state==PLAYER_STATE::JumpDown && *level==LEVEL::Middle && translation.y>BOTTOM+ PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.
		{
			translation.y+=*vel;
		}
		if *state==PLAYER_STATE::JumpUp && *level==LEVEL::Below && translation.y>=MIDDLE+ PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.
		{
			*vel=0.;
			*state=PLAYER_STATE::Run;
			*level=LEVEL::Middle;
		}
		if *state==PLAYER_STATE::JumpUp && *level==LEVEL::Middle && translation.y>=ABOVE+ PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.
		{
			*vel=0.;
			*state=PLAYER_STATE::Run;
			*level=LEVEL::Above;
		}
		if *state==PLAYER_STATE::JumpDown && *level==LEVEL::Above && translation.y<=MIDDLE+ PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.
		{
			*vel=0.;
			*state=PLAYER_STATE::Run;
			*level=LEVEL::Middle;
		}
		if *state==PLAYER_STATE::JumpDown && *level==LEVEL::Middle && translation.y<=BOTTOM+ PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.
		{
			*vel=0.;
			*state=PLAYER_STATE::Run;
			*level=LEVEL::Below;
		}
		if *state==PLAYER_STATE::JumpDown
		{
			*vel=*vel-1.0;
		}
		

	}
}