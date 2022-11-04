package com.example.warehousesrevice.repositories;



import com.example.warehousesrevice.entity.Ware;
import com.example.warehousesrevice.entity.WareRowMapper;
import org.springframework.dao.DataAccessException;
import org.springframework.jdbc.core.PreparedStatementCallback;
import org.springframework.jdbc.core.namedparam.MapSqlParameterSource;
import org.springframework.jdbc.core.namedparam.NamedParameterJdbcTemplate;
import org.springframework.jdbc.core.namedparam.SqlParameterSource;
import org.springframework.jdbc.support.GeneratedKeyHolder;
import org.springframework.jdbc.support.KeyHolder;
import org.springframework.stereotype.Component;

import java.sql.PreparedStatement;
import java.sql.SQLException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@Component
public class WarehouseRepository {


    public WarehouseRepository(NamedParameterJdbcTemplate template) {
        this.template = template;
    }

    NamedParameterJdbcTemplate template;

    public List<Ware> findAll() {
        return template.query("select * from ware.ware", new WareRowMapper());
    }

    public void insertEmployee(Ware emp) {
        final String sql = "insert into ware.ware(id, name , price) values(:id,:name,:price)";

        KeyHolder holder = new GeneratedKeyHolder();
        SqlParameterSource param = new MapSqlParameterSource()
                .addValue("id", emp.getId())
                .addValue("name", emp.getName())
                .addValue("price", emp.getPrice());
        template.update(sql, param, holder);

    }

    public void updateEmployee(Ware emp) {
        final String sql = "update ware.ware set name=:name, price=:price where demo.ware.ware.id=:id";

        KeyHolder holder = new GeneratedKeyHolder();
        SqlParameterSource param = new MapSqlParameterSource()
                .addValue("id", emp.getId())
                .addValue("name", emp.getName())
                .addValue("price", emp.getPrice());
        template.update(sql, param, holder);

    }



    public void deleteEmployee(Ware emp) {
        final String sql = "delete from ware.ware where demo.ware.ware.id=:id";


        Map<String, Object> map = new HashMap<String, Object>();
        map.put("employeeId", emp.getId());

        template.execute(sql, map, new PreparedStatementCallback<Object>() {
            @Override
            public Object doInPreparedStatement(PreparedStatement ps)
                    throws SQLException, DataAccessException {
                return ps.executeUpdate();
            }
        });

    }
}
