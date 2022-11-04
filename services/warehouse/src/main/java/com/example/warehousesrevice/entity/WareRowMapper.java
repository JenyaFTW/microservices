package com.example.warehousesrevice.entity;

import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class WareRowMapper implements RowMapper<Ware> {

    @Override
    public Ware mapRow(ResultSet rs, int arg1) throws SQLException {
        Ware emp = new Ware();
        emp.setName(rs.getString("name"));
        emp.setPrice((rs.getDouble("price")));

        return emp;
    }
}