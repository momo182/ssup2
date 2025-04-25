use std::collections::HashMap;
use std::hash::{Hasher, Hash};
use colored::*;

pub struct Logger {
    prefix: String,
    color: Color,
}

impl Logger {
    pub fn new(prefix: &str) -> Self {
        // Создаем массив доступных цветов
    let colors = vec![
        Color::TrueColor { r: 255, g: 255, b: 104 }, // NO Белый
        Color::TrueColor { r: 255, g: 255, b: 204 }, // Светло-бежевый
        Color::TrueColor { r: 255, g: 255, b: 153 }, // Светло-желтый
        Color::TrueColor { r: 255, g: 255, b: 102 }, // Ярко-желтый
        Color::TrueColor { r: 255, g: 255, b: 51  }, // Темно-желтый
        Color::TrueColor { r: 255, g: 204, b: 255 }, // Светло-розовый
        Color::TrueColor { r: 255, g: 153, b: 255 }, // Розовый
        Color::TrueColor { r: 255, g: 102, b: 255 }, // Ярко-розовый
        Color::TrueColor { r: 255, g: 51,  b: 255 }, // Темно-розовый
        Color::TrueColor { r: 204, g: 255, b: 255 }, // Голубой
        Color::TrueColor { r: 153, g: 255, b: 255 }, // Светло-голубой
        Color::TrueColor { r: 102, g: 255, b: 255 }, // Ярко-голубой
        Color::TrueColor { r: 51,  g: 255, b: 255 }, // Темно-голубой
        Color::TrueColor { r: 255, g: 204, b: 153 }, // Светло-персиковый
        Color::TrueColor { r: 255, g: 153, b: 102 }, // Персиковый
        Color::TrueColor { r: 255, g: 102, b: 51  }, // Ярко-оранжевый
        Color::TrueColor { r: 255, g: 51,  b: 0   }, // Темно-оранжевый
        Color::TrueColor { r: 204, g: 255, b: 204 }, // Светло-зеленый
        Color::TrueColor { r: 153, g: 255, b: 153 }, // Зеленый
        Color::TrueColor { r: 102, g: 255, b: 102 }, // Ярко-зеленый
        Color::TrueColor { r: 51,  g: 255, b: 51  }, // Темно-зеленый
        Color::TrueColor { r: 204, g: 204, b: 255 }, // Светло-фиолетовый
        Color::TrueColor { r: 153, g: 153, b: 255 }, // Фиолетовый
        Color::TrueColor { r: 102, g: 102, b: 255 }, // Ярко-фиолетовый
        Color::TrueColor { r: 51,  g: 51,  b: 255 }, // Темно-фиолетовый
        Color::TrueColor { r: 255, g: 204, b: 204 }, // Светло-красный
        Color::TrueColor { r: 255, g: 153, b: 153 }, // Красный
        Color::TrueColor { r: 255, g: 102, b: 102 }, // Ярко-красный
        Color::TrueColor { r: 255, g: 51,  b: 51  }, // Темно-красный
        Color::TrueColor { r: 255, g: 255, b: 0   }, // Ярко-желтый
        Color::TrueColor { r: 255, g: 0,   b: 0   }, // Чистый красный
        Color::TrueColor { r: 0,   g: 255, b: 0   }, // Чистый зеленый
        Color::TrueColor { r: 0,   g: 0,   b: 255 }, // Чистый синий
    ];

        // Вычисляем хеш от префикса
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        prefix.hash(&mut hasher);
        let hash = hasher.finish();

        // Определяем индекс цвета
        let color_index = hash % colors.len() as u64;
        let color = colors[color_index as usize];

        Logger {
            prefix: prefix.to_string(),
            color,
        }
    }

    pub fn log<T: std::fmt::Display>(&self, msg: T) {
        // Проверяем наличие переменной окружения DEBUG
        let env: HashMap<String, String> = std::env::vars().collect();
        if env.contains_key("DEBUG") {
            // Применяем цвет только к префиксу
            let colored_prefix = self.prefix.color(self.color);
            println!("{}: {}", colored_prefix, msg);
        }
    }
}