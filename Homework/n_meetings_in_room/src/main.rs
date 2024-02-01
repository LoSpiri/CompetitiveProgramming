
fn max_meetings_in_room (meetings: Vec<(i32, i32)>) -> i32 {
    let mut sorted_meetings = meetings.clone();
    sorted_meetings.sort_by(|a, b| a.1.cmp(&b.1));
    let mut count = 0;
    let mut last_meeting_end = 0;
    for meeting in sorted_meetings {
        if meeting.0 > last_meeting_end {
            count += 1;
            last_meeting_end = meeting.1;
        }
    }
    count
}

fn main() {
    let meetings_start = vec![1,3,0,5,8,5];
    let meetings_end = vec![2,4,6,7,9,9];
    let max_meetings = max_meetings_in_room(meetings_start.iter().zip(meetings_end.iter()).map(|(&start, &end)| (start, end)).collect::<Vec<(i32, i32)>>());
    println!("Maximum meetings that can be held in a room: {}", max_meetings);
}
