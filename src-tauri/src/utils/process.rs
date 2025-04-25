use sysinfo::{Pid, System};

pub fn get_children(pid: u32, system: &System) -> tauri::Result<Vec<u32>> {
    
    let mut children = vec![];
    
    children.push(pid);
    
    system.processes().values().into_iter().for_each(|process| {
        if let Some(ppid) = process.parent()  {
            if ppid == Pid::from_u32(pid) {
                if let Ok(ids) = get_children(process.pid().as_u32(), system) {
                    ids.into_iter().for_each(|id| { children.push(id); })
                }
            }
        }
    });
    
    Ok(children)
}