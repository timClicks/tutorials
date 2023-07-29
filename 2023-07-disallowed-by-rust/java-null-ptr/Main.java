public class Main {
    public static void main(String[] args) {
        News news = new News();
        news = null;
        news.whatsHappening();  // This will throw a NullPointerException
    }
}

class News {
    public String whatsHappening() {
        return "Not much";
    }
}

