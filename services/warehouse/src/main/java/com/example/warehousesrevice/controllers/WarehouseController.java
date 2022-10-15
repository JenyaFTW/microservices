package com.example.warehousesrevice.controllers;

import com.example.warehousesrevice.entity.Ware;
import com.example.warehousesrevice.services.WareService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;


@RestController
@RequestMapping(path = "/api/warehouse")
public class WarehouseController {


    @Autowired
    WareService wareService;


    @GetMapping()
    public ResponseEntity<List<Ware>> getAllWares () {
        System.out.println (wareService.getAllWares ());
        return ResponseEntity.ok (wareService.getAllWares ());
    }

    @GetMapping("/{id}")
    public ResponseEntity<Ware> getAllWares (@PathVariable String id) {
        return ResponseEntity.ok (wareService.getAllWares ().get (Integer.parseInt(id)));

    }
}

