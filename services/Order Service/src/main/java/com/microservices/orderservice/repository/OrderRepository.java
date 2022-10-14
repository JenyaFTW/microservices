package com.microservices.orderservice.repository;

import com.microservices.orderservice.entity.Order;
import org.springframework.stereotype.Repository;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

@Repository
public class OrderRepository {

    private List<Order> orders = new ArrayList<>(Arrays.asList(
            new Order(1L, 4L, Arrays.asList(2L, 5L, 5L, 7L)),
            new Order(2L, 2L, Arrays.asList(1L, 1L)),
            new Order(3L, 3L, Arrays.asList(3L, 4L)),
            new Order(4L, 1L, Arrays.asList(3L, 6L))
    ));

    public Order getOrderById(long id) {
        return orders.stream().filter(order -> order.getId().equals(id)).findFirst().orElseThrow();
    }

    public List<Order> getAllOrders (){
        return orders;
    }
}