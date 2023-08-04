use bevy::prelude::*;

use crate::Pheromones;
use crate::FOOD_SPRITE_SCALE;
use crate::INITIAL_FOOD_CAPACITY;
use crate::SPRITE_FOOD;

pub struct ResourcePlugin;

pub struct Food {
    pub x: f32,
    pub y: f32,
    pub food_remaining: i32,
}

#[derive(Component)]
pub struct FoodImageRenderer;

#[derive(Resource)]
pub struct Resources {
    pub foods: Vec<Food>,
}

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Resources::new())
            .add_systems(Startup, setup);
        // .add_systems(
        //     Update,
        //     place_new_food.run_if(on_timer(Duration::from_secs_f32(1.0))),
        // );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pheromones: ResMut<Pheromones>,
    mut resources: ResMut<Resources>,
) {
    for food in resources.foods.iter() {
        pheromones.set_food_pheromone(
            Vec2 {
                x: food.x,
                y: food.y,
            },
            10000.0,
        );

        commands.spawn(SpriteBundle {
            texture: asset_server.load(SPRITE_FOOD),
            transform: Transform::from_xyz(food.x, food.y, 2.0)
                .with_scale(Vec3::splat(FOOD_SPRITE_SCALE)),
            ..Default::default()
        });
    }
}

impl Food {
    pub fn reduce_food(&mut self) {
        self.food_remaining -= 1;
    }
}

impl Resources {
    fn new() -> Self {
        Self {
            foods: {
                vec![
                    Food {
                        x: -750.0,
                        y: 400.0,
                        food_remaining: INITIAL_FOOD_CAPACITY,
                    },
                    Food {
                        x: 100.0,
                        y: 300.0,
                        food_remaining: INITIAL_FOOD_CAPACITY,
                    },
                ]
            },
        }
    }
    pub fn remove_food(&mut self, mut pheromones: ResMut<Pheromones>) {
        self.foods.retain(
            |Food {
                 food_remaining,
                 x,
                 y,
             }| {
                if food_remaining > &0 {
                    true
                } else {
                    pheromones.set_food_pheromone(Vec2 { x: *x, y: *y }, 0.0);

                    false
                }
            },
        );
    }
}
