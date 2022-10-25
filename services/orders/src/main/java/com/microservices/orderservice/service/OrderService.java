package com.microservices.orderservice.service;

import com.microservices.orderservice.entity.Order;
import com.microservices.orderservice.repository.OrderRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class OrderService {

    private final OrderRepository orderRepository;

    @Autowired
    public OrderService(OrderRepository orderRepository) {
        this.orderRepository = orderRepository;
    }

    public Order getOrderById(Long id) {
        return orderRepository.findById(id).orElse(null);
    }

    public List<Order> getAllOrders() {
        return orderRepository.findAll();
    }

    public void addOrder (Order order) {
        orderRepository.save(order);
    }

    public void updateOrder (Order order) {
        orderRepository.save(order);
    }

    public void deleteOrder (Long id) {
        orderRepository.deleteById(id);
    }
}