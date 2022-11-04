package com.example.warehousesrevice.services;

import com.example.warehousesrevice.entity.Ware;
import com.example.warehousesrevice.repositories.WarehouseRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class WareService {


    @Autowired
    private WarehouseRepository warehouseRepository;


    public List<Ware> getAllWares () {
        return warehouseRepository.findAll();
    }

    public void addWare (Ware ware) {
        warehouseRepository.insertEmployee(ware);
    }

    public void updateWare (Ware ware) {
        warehouseRepository.updateEmployee(ware);
    }

    public void deleteWare (Ware ware) {
        warehouseRepository.deleteEmployee(ware);
    }

    public Ware getWareById (Long id) {
        return warehouseRepository.findAll().get(Math.toIntExact(id));
    }



}
