use crate::rewards::*;
use crate::chasers::*;

use std::ops::Sub;


pub fn chase_closest(chaser: &mut Chaser, rewards: &Vec<Reward>){

    if rewards.len() == 0 { 
       return; 
    }

    // find nearest reward
    let mut shortest_distance = 100000000.0;

    // find closest target
    for reward in rewards {

        let distance = reward.position.distance(chaser.position);

        if reward.position.distance(chaser.position) < shortest_distance {    
            chaser.direction = reward.position.sub(chaser.position).normalize();
            shortest_distance = distance;
            chaser.target_id = reward.id; 
        }   
    }
}