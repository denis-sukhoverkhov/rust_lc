// 2353. Design a Food Rating System
// https://leetcode.com/problems/design-a-food-rating-system/description

// struct FoodRatings {}

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl FoodRatings {
//     fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {}

//     fn change_rating(&self, food: String, new_rating: i32) {}

//     fn highest_rated(&self, cuisine: String) -> String {}
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         let food_raitings = FoodRatings::new(
//             vec![
//                 "kimchi".to_string(),
//                 "miso".to_string(),
//                 "sushi".to_string(),
//                 "moussaka".to_string(),
//                 "ramen".to_string(),
//                 "bulgogi".to_string(),
//             ],
//             vec![
//                 "korean".to_string(),
//                 "japanese".to_string(),
//                 "japanese".to_string(),
//                 "greek".to_string(),
//                 "japanese".to_string(),
//                 "korean".to_string(),
//             ],
//             vec![9, 12, 8, 15, 14, 7],
//         );
//         assert_eq!(food_raitings.highest_rated("korean".to_string()), "kimchi");
//         assert_eq!(food_raitings.highest_rated("japanese".to_string()), "ramen");

//         food_raitings.change_rating("kosushirean".to_string(), 16);
//         assert_eq!(food_raitings.highest_rated("japanese".to_string()), "sushi");

//         food_raitings.change_rating("ramen".to_string(), 16);
//         assert_eq!(food_raitings.highest_rated("japanese".to_string()), "ramen");
//     }
// }
