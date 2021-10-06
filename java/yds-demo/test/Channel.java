import java.io.File;
import java.io.FileInputStream;
import java.io.RandomAccessFile;
import java.nio.ByteBuffer;
import java.nio.channels.FileChannel;

public class Channel {
    public static void main(String[] args) {
        FileInputStream fin = null;
        FileChannel fChannel = null;
        try {
            fin = new FileInputStream("F:/workspace/java/test/pom.txt");
            fChannel = fin.getChannel();
            ByteBuffer bbf = ByteBuffer.allocate(64);
    
            while(fChannel.read(bbf)!=-1){
                int limit = bbf.limit();
                bbf.flip();
                while(limit!=0 && bbf.hasRemaining()){
                   System.out.print(bbf.get());
                }
                bbf.clear();
            }
            
        } catch (Exception e) {
            System.out.println("文件不存在");
        }finally{
           
        }

    }
    
}
