#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec, Map, String};

#[contract]
pub struct SocialMediaContract;

#[contractimpl]
impl SocialMediaContract {

    // Add a new post
    pub fn create_post(env: Env, user: String, content: String) {
        let mut posts: Map<String, Vec<String>> = env
            .storage()
            .instance()
            .get(&Symbol::short("POSTS"))
            .unwrap_or(Map::new(&env));

        let mut user_posts = posts.get(user.clone()).unwrap_or(Vec::new(&env));

        user_posts.push_back(content);
        posts.set(user, user_posts);

        env.storage().instance().set(&Symbol::short("POSTS"), &posts);
    }

    // Get posts of a user
    pub fn get_posts(env: Env, user: String) -> Vec<String> {
        let posts: Map<String, Vec<String>> = env
            .storage()
            .instance()
            .get(&Symbol::short("POSTS"))
            .unwrap_or(Map::new(&env));

        posts.get(user).unwrap_or(Vec::new(&env))
    }
}