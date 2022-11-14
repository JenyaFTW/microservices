package com.example.warehousesrevice.controllers;

import com.example.warehousesrevice.entity.Ware;
import com.example.warehousesrevice.services.WareService;
import com.google.gson.Gson;
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

    @GetMapping("get/{id}")
    public ResponseEntity<Ware> getAllWares (@PathVariable String id) {
        return ResponseEntity.ok (wareService.getAllWares ().get (Integer.parseInt(id)));
    }

    @DeleteMapping("delete/{id}")
    public void deleteWare (@PathVariable String id) {
        wareService.deleteWare (wareService.getWareById(Long.parseLong(id)));
    }

    @PostMapping("update/{id}")
    public void updateWare (@PathVariable String id, @RequestBody String json) {
        Gson gson = new Gson ();
        Ware ware = gson.fromJson (json, Ware.class);
        Ware previousWare = wareService.getWareById(Long.parseLong(id));
        previousWare.setName(ware.getName ());
        previousWare.setPrice(ware.getPrice ());
        wareService.updateWare (ware);
    }

    @PostMapping("create")
    public void createWare (@RequestBody String json) {
        Gson gson = new Gson ();
        Ware ware = gson.fromJson (json, Ware.class);
        wareService.addWare(ware);
    }

}

