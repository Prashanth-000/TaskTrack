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

    add_task(&mut tasks,3,"task_3".to_string());

    list_task(&tasks);

    complete_status(&mut tasks,1);

    list_task(&tasks);

    println!("Total Number of Tasks={}",tasks.len());

    if(delete_task(&mut tasks, 2)){
        println!("Deleted");
    }
    else{
        println!("Not deleted");
    }

    println!("Total Number of Tasks={}",tasks.len());
    
    list_task(&tasks);


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

fn add_task(tasks:&mut Vec<Task>,id:u32,title:String){
    let task=Task{
        id,
        title,
        status:Status::Pending
    };

    tasks.push(task);
}

fn list_task(tasks:&[Task]){
    for task in tasks{
        let status_text=match task.status{
            Status::Pending=>"Pending",
            Status::Completed=>"Completed"
        };

        println!("{} | {} | {} |",task.id,task.title,status_text);
        println!("\n");
    }
}

fn complete_status(tasks:&mut [Task],id:u32)->bool{
    for task in tasks{
        if task.id==id{
            task.status=Status::Completed;
            return true
        }
    }
    return false
}

fn delete_task(tasks: &mut Vec<Task>,id:u32)-> bool{
    let mut index=0;

    while index<tasks.len(){
        if tasks[index].id==id{
            tasks.remove(index);
            return true;
        }

        index +=1;
    }

    return false;
}