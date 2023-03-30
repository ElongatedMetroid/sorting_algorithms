use bevy::prelude::*;
use sorting::SortingAlgorithm;

pub struct SortVisualizePlugin;

impl Plugin for SortVisualizePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SortingVector(vec![5, 4, 3, 2, 1, -12, 20, -23, 90, -20, 032, -23, 2, 1, 3, 5, 23, 12, 2]))
            .insert_resource(UpdateTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_startup_system(setup)
            .add_system(update);
    }
}

#[derive(Resource)]
pub struct UpdateTimer(Timer);
#[derive(Resource)]
pub struct SortingVector(Vec<isize>);

#[derive(Component)]
pub struct SortingColumn(usize);

pub fn setup(
    mut commands: Commands,
    sorting_vector: Res<SortingVector>
) {
    for index in 0..sorting_vector.0.len() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(10.0, 1.0)),
                    ..default()
                },
                // TODO have the columns fill up the whole screen
                transform: Transform::from_xyz(index as f32 * 10.0, 0.0, 2.0),
                ..default()
            },
            SortingColumn(index),
        ));
    }
}

fn update(
    time: Res<Time>,
    mut timer: ResMut<UpdateTimer>,
    mut sorting_vector: ResMut<SortingVector>,
    mut sorting_columns: Query<(&mut Sprite, &SortingColumn)>
) {
    if timer.0.tick(time.delta()).just_finished() {
        if sorting_vector.0.step() {
            println!("sorting completed");
            return;
        };
        
        for (mut sprite, column) in &mut sorting_columns {
            let new_height = sorting_vector.0[column.0] as f32 * 10.0;

            // if the old height was greater than the new height
            if sprite.custom_size.unwrap().y > new_height {
                sprite.color = Color::BLUE;
            } else {
                sprite.color = Color::RED;
            }

            sprite.custom_size = Some(Vec2::new(10.0, new_height));
        }
    }
}