package com.microservices.orderservice.controller;

import com.microservices.orderservice.entity.Order;
import com.microservices.orderservice.service.OrderService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/orders")
public class OrderController {

    private final OrderService orderService;

    @Autowired
    public OrderController(OrderService orderService) {
        this.orderService = orderService;
    }

    @GetMapping("get/{orderId}")
    @ResponseBody
    public Order getOrderById(@PathVariable long orderId) {
        return orderService.getOrderById(orderId);
    }

    @GetMapping()
    @ResponseBody
    public List<Order> getAllOrders() {
        return orderService.getAllOrders();
    }

    @PostMapping("create")
    public void createOrder (@RequestBody Order order) {
        orderService.addOrder(order);
    }

    @DeleteMapping("delete/{orderId}")
    public void deleteOrder (@PathVariable Long orderId) {
        orderService.deleteOrder(orderId);
    }

    @PutMapping("update/{orderId}")
    public void updateOrder (@RequestBody Order order, @PathVariable Long orderId) {
        if (order.getId() == null) order.setId(orderId);
        orderService.updateOrder(order);
    }

}