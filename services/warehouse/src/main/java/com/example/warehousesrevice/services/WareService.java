package com.example.warehousesrevice.services;

import com.example.warehousesrevice.entity.Ware;
import com.example.warehousesrevice.repositories.WarehouseRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class WareService {


    private WarehouseRepository warehouseRepository = new WarehouseRepository ();


    public List<Ware> getAllWares () {
        return warehouseRepository.getAllWares ();
    }



}
