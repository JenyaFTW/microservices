package com.microservices.orderservice.controller;

import com.microservices.orderservice.entity.Order;
import com.microservices.orderservice.service.OrderService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.client.RestTemplate;

import java.util.List;

@CrossOrigin
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
    public ResponseEntity<Order> getOrderById(@PathVariable long orderId) {
        Order order = orderService.getOrderById(orderId);
        if (order == null)
            return new ResponseEntity<>(HttpStatus.NO_CONTENT);
        else
            return ResponseEntity.ok(order);
    }

    @GetMapping()
    @ResponseBody
    public ResponseEntity<List<Order>> getAllOrders() {
        return ResponseEntity.ok(orderService.getAllOrders());
    }

    @PostMapping("create")
    public ResponseEntity createOrder(@RequestBody Order order) {
        RestTemplate restTemplate = new RestTemplate();
        if (!restTemplate.getForEntity("http://localhost:3003/api/customer/" + order.getClientId(), Object.class).hasBody()) {
            System.out.println("No such customer");
            return new ResponseEntity<>(HttpStatus.NOT_ACCEPTABLE);
        }
        for (Long itemId : order.getItemId()) {
            if (!restTemplate.getForEntity("http://localhost:3001/api/warehouse/get/" + itemId, Object.class).hasBody()) {
                System.out.println("No such item in warehouse");
                return new ResponseEntity<>(HttpStatus.NOT_ACCEPTABLE);
            }
        }
        orderService.addOrder(order);
        return ResponseEntity.ok(HttpStatus.ACCEPTED);
    }

    @DeleteMapping("delete/{orderId}")
    public ResponseEntity deleteOrder(@PathVariable Long orderId) {
        Order order = orderService.getOrderById(orderId);
        if (order == null)
            return new ResponseEntity<>(HttpStatus.NOT_ACCEPTABLE);
        orderService.deleteOrder(orderId);
        return ResponseEntity.ok(HttpStatus.ACCEPTED);
    }

    @PutMapping("update/{orderId}")
    public ResponseEntity updateOrder(@RequestBody Order order, @PathVariable Long orderId) {
        if (order.getId() == null) order.setId(orderId);
        Order order1 = orderService.getOrderById(orderId);
        if (order1 == null)
            return new ResponseEntity<>(HttpStatus.NOT_ACCEPTABLE);
        orderService.updateOrder(order);
        return ResponseEntity.ok(HttpStatus.ACCEPTED);
    }

}
