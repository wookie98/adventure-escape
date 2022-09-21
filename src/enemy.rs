use std::cmp;

use crate::components::{ Movable, SpriteSize, Velocity, LEVEL,PlayerLevel,ENEMY_TYPE, EnemyType, Enemy,Lives};
use crate::{
	GameTextures, WinSize, SAMURAI_SIZE,NINJA_SIZE,BOTTOM,MIDDLE,ABOVE, SPRITE_SCALE, EnemyCount, ENEMY_MAX, ENEMY_SIZE, Chance
};
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::Rng;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(enemy_spawn_system),
        );
	}

}

fn enemy_spawn_system(
	mut commands: Commands,
	game_textures: Res<GameTextures>,
	mut enemy_count: ResMut<EnemyCount>,
	win_size: Res<WinSize>,
    mut chance: ResMut<Chance>,
) {
	if enemy_count.0 < ENEMY_MAX {
        let mut rng = rand::thread_rng();
        
        if rng.gen_range(0..100)<chance.0
        {
            
            let i=rng.gen_range(0..3);
            let mut level:LEVEL=LEVEL::Below;
            let mut y=BOTTOM+ ENEMY_SIZE.1 / 2. * SPRITE_SCALE + 5.;
            if i==1
            {
                level=LEVEL::Middle;
                y=MIDDLE+ ENEMY_SIZE.1 / 2. * SPRITE_SCALE + 5.;
            }
            else if i==2
            {
                level=LEVEL::Above;
                y=ABOVE+ ENEMY_SIZE.1 / 2. * SPRITE_SCALE + 5.;
            }
            let i=rng.gen_range(0..2);
            let mut enType=ENEMY_TYPE::SAMURAI;
            let mut vel=(0.,0.);
            let mut life=3;
            let mut textureOpt=game_textures.samurai.clone();
            if i==1
            {
               enType=ENEMY_TYPE::NINJA;
               vel=(-0.5,0.);
               life=1;
               textureOpt=game_textures.ninja.clone();
            }

		commands
			.spawn_bundle(SpriteBundle {
				texture: textureOpt,
				transform: Transform {
					translation: Vec3::new(300., y, 10.),
					scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Enemy)
			.insert(SpriteSize::from(ENEMY_SIZE))
            .insert(PlayerLevel{level:level})
            .insert(EnemyType{Etype:enType})
            .insert(Velocity{x:vel.0,y:vel.1})
            .insert(Movable { auto_despawn: true, enemy:true, ammo:false })
            .insert(Lives { lives:life });

		enemy_count.0 += 1;
        }
	}
}