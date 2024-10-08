use std::fs;

use crate::dtos::product_dto::ProductDto;

pub fn get_products() -> Vec<ProductDto> {
    let file = fs::read_to_string("src/data/productos.json").unwrap();

    let products: Vec<ProductDto> = serde_json::from_str(file.as_str()).unwrap();

    return products;
}

pub fn add_product(product_dto: ProductDto) {
    let mut products = get_products();

    products.push(product_dto);

    let json = serde_json::to_string_pretty(&products).unwrap();
    
    fs::write("src/data/productos.json", json).unwrap();
}