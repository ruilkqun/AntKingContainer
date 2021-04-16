use uuid::Uuid;

pub fn computer_container_id() -> String{
    let mut container_id:String = "".to_string();

    let container_uuid_1 = format!("{}",Uuid::new_v4());
    let container_uuid_2:Vec<&str> = container_uuid_1.split('-').collect();
    for v in container_uuid_2 {
        container_id += v
    }

    let container_uuid_1 = format!("{}",Uuid::new_v4());
    let container_uuid_2:Vec<&str> = container_uuid_1.split('-').collect();
    for v in container_uuid_2 {
        container_id += v
    }

    return container_id
}