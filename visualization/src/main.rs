use bevy::prelude::*;
use sorting::bubble_sort::BubbleSort;
use visualization::sort_visualize::SortVisualizePlugin;

fn main() {
    let mut vec = vec!['z', 'a', 'c', 'd'];
    vec.sort();
    println!("{:?}", vec);

    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_plugin(SortVisualizePlugin)
    //     .run();
}
