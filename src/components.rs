use bevy::math::{Vec2, Vec3};
use bevy::prelude::Component;
use bevy::time::Timer;

// region:    --- Common Components
#[derive(Component)]
pub struct Velocity {
	pub x: f32,
	pub y: f32,
}

#[derive(Component)]
pub struct Movable {
	pub auto_despawn: bool,
	pub enemy:bool,
	pub ammo:bool
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Ammo
{
    pub ammo: i32,
}

#[derive(Component)]
pub struct Lives
{
    pub lives: i32,
}


#[derive(Component)]
pub struct  PlayerLevel
{
	pub level:LEVEL,
}
#[derive(PartialEq)]
pub enum LEVEL
{
	Middle,
	Above,
	Below
}

#[derive(Component)]
pub struct  PlayerState
{
	pub state:PLAYER_STATE,
}
#[derive(PartialEq)]
pub enum PLAYER_STATE
{
	Run,
	JumpUp,
	JumpDown,
}

#[derive(Component)]
pub struct  EnemyType
{
	pub Etype:ENEMY_TYPE,
}
#[derive(PartialEq)]
pub enum ENEMY_TYPE
{
	NINJA,
	SAMURAI,
}

#[derive(Component)]
pub struct TopText;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
	fn from(val: (f32, f32)) -> Self {
		SpriteSize(Vec2::new(val.0, val.1))
	}
}

// endregion: --- Common Components

// region:    --- Player Components
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;
// endregion: --- Player Components
#[derive(Component)]
pub struct AmmoDrop;
// region:    --- Enemy Components
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromEnemy;
// endregion: --- Enemy Components


