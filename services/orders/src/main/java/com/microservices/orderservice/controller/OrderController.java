package com.microservices.orderservice.controller;

import com.microservices.orderservice.entity.Order;
import com.microservices.orderservice.service.OrderService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.MediaType;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@Controller
@RequestMapping("/api/orders")
public class OrderController {

    private OrderService orderService;

    public OrderService getOrderService() {
        return orderService;
    }

    @Autowired
    public void setOrderService(OrderService orderService) {
        this.orderService = orderService;
    }

    @GetMapping("/{orderId}")
    @ResponseBody
    public Order getOrderById(@PathVariable long orderId) {
        return orderService.getOrderById(orderId);
    }

    @GetMapping()
    @ResponseBody
    public List<Order> getAllOrders() {
        return orderService.getAllOrders();
    }

}