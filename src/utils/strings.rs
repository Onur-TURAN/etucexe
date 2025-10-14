// Bir dosya yolunu al, dosyayı aç, tüm içeriğini byte'lar halinde belleğe yükle
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

// Bulunan string'leri saklamak için bir liste oluştur
// Geçici olarak karakter biriktirmek için boş bir string oluştur


// Dosyadaki byte'ları sırayla dolaş
// Eğer byte yazdırılabilir ASCII karakterse veya boşluksa geçici string'e ekle
// Yoksa (yazdırılamayan bir şey geldiyse):
//   - Eğer geçici string en az 4 karakterse onu sonuç listesine ekle
//   - Geçici string'i sıfırla


// Döngü bittiğinde hala geçici string varsa ve uzunluğu >= 4 ise listeye ekle


// Sonuç listesini döndür


// pub fonksiyonu: extract_strings fonksiyonunu çağır
// Dönen string'leri ekrana yazdır

