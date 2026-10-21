# Subscription source

Where the library comes from: Local subscriptions, Miniflux or FreshRSS, and the provider connection that keeps a provider library in step.

## Boundary

It owns choosing the source and connecting or disconnecting a provider. It does not own the feeds inside a library or refreshing them, which belong to Feed management.
