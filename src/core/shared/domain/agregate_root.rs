use super::domain_events::DomainEvent;

pub trait AggregateRoot {
    fn domain_events(&self) -> &Vec<Box<dyn DomainEvent>>;

    fn domain_events_mut(&mut self) -> &mut Vec<Box<dyn DomainEvent>>;

    fn record_event(&mut self, event: Box<dyn DomainEvent>) {
        self.domain_events_mut().push(event);
    }

    fn pull_domain_events(&mut self) -> Vec<Box<dyn DomainEvent>> {
        std::mem::take(self.domain_events_mut())
    }
}
