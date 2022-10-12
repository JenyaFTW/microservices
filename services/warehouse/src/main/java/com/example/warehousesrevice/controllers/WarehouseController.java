package com.example.warehousesrevice.controllers;

import com.example.warehousesrevice.entity.Ware;
import com.example.warehousesrevice.services.WareService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;



@RestController
public class WarehouseController {


    @Autowired
    WareService wareService;


//    @GetMapping
//    @RequestMapping(path = "/wares", produces= MediaType.APPLICATION_JSON_VALUE)
//    public ResponseEntity<List<Ware>> getAllWares () {
//        System.out.println (wareService.getAllWares ());
//        return new ResponseEntity (wareService.getAllWares (), HttpStatus.OK);
//    }

    @GetMapping
    @RequestMapping(path = "/ware", produces= MediaType.APPLICATION_JSON_VALUE)
    public ResponseEntity<Ware> getAllWares (@RequestParam int id) {
        return ResponseEntity.ok (wareService.getAllWares ().get (id));

    }
}
