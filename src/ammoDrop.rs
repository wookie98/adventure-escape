use std::cmp;

use crate::components::{ Movable, SpriteSize, Velocity, LEVEL,PlayerLevel, AmmoDrop};
use crate::{
	GameTextures, WinSize, AMMO_SIZE,BOTTOM,MIDDLE,ABOVE, SPRITE_SCALE,Chance,
};
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::Rng;

pub struct AmmoDropsPlugin;

impl Plugin for AmmoDropsPlugin {
	fn build(&self, app: &mut App) {
		app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(bullet_spawn_system),
        );
	}

}

fn bullet_spawn_system(
	mut commands: Commands,
	game_textures: Res<GameTextures>,
	win_size: Res<WinSize>,
    chance: ResMut<Chance>,
) {
	
        let mut rng = rand::thread_rng();
        
        if rng.gen_range(0..100)<chance.0-5
        {
            
            let i=rng.gen_range(0..3);
            let mut level:LEVEL=LEVEL::Below;
            let mut y=BOTTOM+ AMMO_SIZE.1 / 2. * SPRITE_SCALE + 5.;
            if i==1
            {
                level=LEVEL::Middle;
                y=MIDDLE+ AMMO_SIZE.1 / 2. * SPRITE_SCALE + 5.;
            }
            else if i==2
            {
                level=LEVEL::Above;
                y=ABOVE+ AMMO_SIZE.1 / 2. * SPRITE_SCALE + 5.;
            }

		commands
			.spawn_bundle(SpriteBundle {
				texture: game_textures.ammo.clone(),
				transform: Transform {
					translation: Vec3::new(300., y, 10.),
					scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
					..Default::default()
				},
				..Default::default()
			})
	        .insert(Movable { auto_despawn: true, enemy:false, ammo:true })
            .insert(Velocity{x:0.0,y:0.})
            .insert(SpriteSize::from(AMMO_SIZE))
            .insert(AmmoDrop);
        }
	
}