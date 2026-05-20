mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::Petstore;
    export! {Petstore}
}

use crate::bindings::exports::wasco_dev::petstore::petstore::{
    AuthError, Guest, ResourceError,
};

struct Petstore;

impl Guest for Petstore {
    /// GET /pets - List all pets
    ///
    /// Returns: JSON array of pet objects
    fn list_pets(api_key: String) -> Result<String, AuthError> {
        // TODO: Implement API call to GET /pets
        // Example response: [{"id": "1", "name": "Fluffy", "tag": "cat"}]
        todo!("Implement GET /pets with api_key: {}", api_key)
    }

    /// GET /pets/{petId} - Get a pet by ID
    ///
    /// Parameters:
    /// - api_key: API key for authentication
    /// - pet_id: Pet identifier
    ///
    /// Returns: JSON pet object
    fn get_pet_by_id(api_key: String, pet_id: String) -> Result<String, ResourceError> {
        // TODO: Implement API call to GET /pets/{petId}
        // Example response: {"id": "1", "name": "Fluffy", "tag": "cat"}
        todo!("Implement GET /pets/{} with api_key: {}", pet_id, api_key)
    }

    /// POST /pets - Create a pet
    ///
    /// Parameters:
    /// - api_key: API key for authentication
    /// - body: JSON pet object (e.g., {"name": "Fluffy", "tag": "cat"})
    ///
    /// Returns: JSON pet object with ID
    fn create_pet(api_key: String, body: String) -> Result<String, ResourceError> {
        // TODO: Implement API call to POST /pets
        // Parse the body JSON, make the request, return the created pet
        todo!("Implement POST /pets with api_key: {}, body: {}", api_key, body)
    }

    /// PUT /pets/{petId} - Update a pet
    ///
    /// Parameters:
    /// - api_key: API key for authentication
    /// - pet_id: Pet identifier
    /// - body: JSON pet object (e.g., {"name": "Fluffy", "tag": "cat"})
    ///
    /// Returns: JSON updated pet object
    fn update_pet(api_key: String, pet_id: String, body: String) -> Result<String, ResourceError> {
        // TODO: Implement API call to PUT /pets/{petId}
        // Parse the body JSON, make the request, return the updated pet
        todo!("Implement PUT /pets/{} with api_key: {}, body: {}", pet_id, api_key, body)
    }

    /// DELETE /pets/{petId} - Delete a pet
    ///
    /// Parameters:
    /// - api_key: API key for authentication
    /// - pet_id: Pet identifier
    ///
    /// Returns: Empty string on success
    fn delete_pet(api_key: String, pet_id: String) -> Result<String, ResourceError> {
        // TODO: Implement API call to DELETE /pets/{petId}
        // Return empty string on successful deletion
        todo!("Implement DELETE /pets/{} with api_key: {}", pet_id, api_key)
    }
}
