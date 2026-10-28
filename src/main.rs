fn main() {

    let mut tasks: Vec<Task>= Vec::new();

    tasks.push(
        Task{
        id:1,
        title:"task_1".to_string(),
        status: Status::Pending
    }
    );

        tasks.push(
        Task{
        id:2,
        title:"task_2".to_string(),
        status: Status::Completed
    }
    );

    println!("Total Number of Tasks={}",tasks.len());

    for task in &tasks{
        let status_text=match task.status{
            Status::Pending=>"Pending",
            Status::Completed=>"Completed"
        };

        println!("{} | {} | {} |",task.id,task.title,status_text);
    }

    println!("Total Number of Tasks={}",tasks.len());

}

struct Task{
    id: u32,
    title: String,
    status: Status
}

enum Status{
    Pending,
    Completed
}