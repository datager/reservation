# Core Reservation Service

- Feature Name: core-reservation
- Start Date: 2023-12-23 00:00:08
- RFC PR: [rust-lang/rfcs#0000](https://github.com/rust-lang/rfcs/pull/0000)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)

## Summary

A core reservation service that solves the problem of reserving a resource for a period of time. We leverage postgres EXCLUDE constraints to ensure that only one reservation can be made tor a given resource at a gaven time .

## Motivation

We need a common solution for various reservation requirements: 1) calendar booking: 2) hotel/room booking; 3)meeting room booking: 4) parking lot booking: 5) etc. Repeatedly building features for these requirements is a waste of time and resources. We should have a common solution that can be used by all teams .

## Guide-level explanation

Basic architecture:

![basic arch](images/arch1.png)

### Service interface

We would use gRPC as a service interface. Below is the proto definition:

```proto
enum ReservationStatus {
    UNKNOWN = 0;  // 未知
    PENDING = 1;  // 等待中
    CONFIRMED = 2; // 已确认
    BLOCKED = 3;  // 已拒绝
}

message Reservation {
    string id = 1;
    string user_id = 2;
    ReservationStatus status = 3;

    // resource reservation window
    string resource_id = 4;
    google.protobuf.Timestamp start = 5;
    google.protobuf.Timestamp end = 6;

    // extra note
    string note = 7;
}

message ReserveRequest {
    Reservation reservation = 1;
}

message ReserveResponse {
    Reservation reservation = 1;
}

message UpdateRequest {
    ReservationStatus status = 1;
    string note = 2;
}

message UpdateResponse {
    Reservation reservation = 1;
}

message ConfirmRequest {
    string id = 1;
}

message ConfirmResponse {
    Reservation reservation = 1;
}

message CancelRequest {
    string id = 1;
}

message CancelResponse {
    Reservation reservation = 1;
}

message GetRequest {
    string id = 1;
}

message GetResponse {
    Reservation reservation = 1;
}

message QueryRequest {
    string resource_id = 1;
    string user_id = 2;
    // use status to filter result. IF UNKNOWN, return all reservations
    ReservationStatus status = 3;
    google.protobuf.Timestamp start = 3;
    google.protobuf.Timestamp end = 4;
}

service ReservationService {
    rpc reserve(ReserveRequest) returns (ReserveResponse);
    rpc confirm(ConfirmRequest) returns (ConfirmResponse);
    rpc update(UpdateRequest) returns (Reservation);
    rpc cancel(CancelRequest) returns (CancelResponse);
    rpc get(getRequest) returns (getResponse);
    rpc query(Reservation) returns (stream Reservation);
}
```
