use super::*;
use std::thread;
use std::time;

#[test]
fn pub_sub_create() {
    let message = Message::new(String::from(""));
    assert_eq!(message.data, String::from(""));
    let mut topic = Topic::new("topic", Duration::seconds(60), Duration::seconds(0));
    topic.set_message_ttl(Duration::seconds(120));
    let topic_meta = TopicMeta::from(&topic);
    assert_eq!(topic_meta.name, String::from("topic"));
    assert_eq!(topic_meta.message_ttl, 120);
    let mut subscription = Subscription::new_head(
        "subscription",
        &topic,
        Duration::seconds(60),
        Duration::seconds(0),
    );
    subscription.set_ack_deadline(Duration::seconds(120));
    let subscription_meta = SubscriptionMeta::from(&subscription);
    assert_eq!(subscription_meta.name, String::from("subscription"));
    assert_eq!(subscription_meta.topic, String::from("topic"));
    assert_eq!(subscription_meta.ack_deadline, 120);
}

#[test]
fn pub_sub_basic() {
    let mut topic = Topic::new("topic", Duration::seconds(60), Duration::seconds(0));
    let mut subscription = Subscription::new_head(
        "subscription",
        &topic,
        Duration::milliseconds(10),
        Duration::seconds(0),
    );
    topic.publish(String::from("a"));
    topic.publish(String::from("b"));
    assert_eq!(2, topic.len());
    let message = subscription.pull().unwrap();
    assert_eq!(String::from("a"), message.data);
    subscription.ack(message.id);
    let message = subscription.pull().unwrap();
    assert_eq!(None, subscription.pull());
    subscription.ack_many(&[message.id]);
    assert_eq!(String::from("b"), message.data);
    assert_eq!(None, subscription.pull());
    thread::sleep(time::Duration::from_millis(20));
    assert_eq!(None, subscription.pull());

    let mut subscription = Subscription::new_tail(
        "subscription",
        &topic,
        Duration::milliseconds(10),
        Duration::seconds(0),
    );
    assert_eq!(None, subscription.pull());

    topic.publish(String::from("c"));
    let message = subscription.pull().unwrap();
    assert_eq!(String::from("c"), message.data);
    thread::sleep(time::Duration::from_millis(20));
    let message = subscription.pull().unwrap();
    assert_eq!(String::from("c"), message.data);
    subscription.ack_many(&[message.id]);
    thread::sleep(time::Duration::from_millis(20));
    assert_eq!(None, subscription.pull());

    topic.publish(String::from("d"));
    topic.set_message_ttl(Duration::milliseconds(10));
    thread::sleep(time::Duration::from_millis(20));
    topic.cleanup();
    assert_eq!(None, subscription.pull());
}
