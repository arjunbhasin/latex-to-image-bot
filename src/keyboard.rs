use teloxide::types::{
    InlineKeyboardButton, 
    InlineKeyboardMarkup,
};

pub async fn construct_main_menu_keyboard() -> InlineKeyboardMarkup {
    let categories = vec![
        "Arithmetic", 
        "Algebra", 
        // "Calculus", 
        // "Linear Algebra", 
        // "Set Theory"
    ];
    construct_keyboard_from_vector(categories).await
}

pub async fn construct_arithmetic_keyboard() -> InlineKeyboardMarkup {
    let categories = vec![
        "Common Expressions", 
        "Basic Concepts", 
    ];

    construct_keyboard_from_vector(categories).await
}

async fn construct_keyboard_from_vector(categories: Vec<&str>) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for cat in categories.chunks(3) {
        let row = cat
            .iter()
            .map(|&cat| InlineKeyboardButton::callback(cat.to_owned(), cat.to_owned()))
            .collect();

        keyboard.push(row);
    }
    let keyboard = InlineKeyboardMarkup::new(keyboard);
    
    keyboard
}