import inh_gr;

void db(int I=1, int J=1, pen p=blue, pen p1=p, pair sh = (0,0)){
	path b = box((0,0),(1,1));
	for (int i = 1; i <= I; ++i){
		for (int j = 1; j <= J; ++j){
			filldraw(shift(sh+(i-1,j-1))*b, p, fillpen=p1); 
			} 
		}
}

for (int i; i<9;++i){
	//write(i%3,i#3);
	db(I=10,J=10, p=black, p1=orange, sh=((i%3)*dx,-(i#3)*dy));
	string s_i = (string) (i+1);
	shipout("fig"+s_i+"00");	
	//erase();
}



