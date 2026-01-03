**SQLバインド**

SQLクエリにパラメータを動的に挿入する方法の一つです。これにより、SQLインジェクション攻撃を防ぎ、クエリの実行効率を向上させることができます。Javaでは、PreparedStatementを使用してSQLバインドを行います。

```
import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.PreparedStatement;
import java.sql.ResultSet;

public class Example {
    public static void main(String[] args) {
        String url = "jdbc:mysql://localhost:3306/mydatabase";
        String user = "username";
        String password = "password";

        try (Connection conn = DriverManager.getConnection(url, user, password)) {
            String sql = "SELECT * FROM users WHERE username = ? AND age = ?";
            PreparedStatement pstmt = conn.prepareStatement(sql);
            pstmt.setString(1, "john_doe");
            pstmt.setInt(2, 30);

            ResultSet rs = pstmt.executeQuery();
            while (rs.next()) {
                System.out.println("User ID: " + rs.getInt("id"));
                System.out.println("Username: " + rs.getString("username"));
                System.out.println("Age: " + rs.getInt("age"));
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}

```
