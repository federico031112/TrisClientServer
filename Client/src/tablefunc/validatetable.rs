pub fn valida_tabella(tabella: &[[char; 3]; 3]) -> bool {
    if tabella[0][0] == tabella[0][1] && tabella[0][1] == tabella[0][2] && (tabella[0][0] == 'X' || tabella[0][0] == 'O'){
        return true;
    }else if tabella[1][0] == tabella[1][1] && tabella[1][1] == tabella[1][2] && (tabella[1][0] == 'X' || tabella[1][0] == 'O'){
        return true;
    }else if tabella[2][0] == tabella[2][1] && tabella[2][1] == tabella[2][2] && (tabella[2][0] == 'X' || tabella[2][0] == 'O'){
        return true;
    }else if tabella[0][0] == tabella[1][0] && tabella[1][0] == tabella[2][0] && (tabella[0][0] == 'X' || tabella[0][0] == 'O'){
        return true;
    }else if tabella[0][1] == tabella[1][1] && tabella[1][1] == tabella[2][1] && (tabella[0][1] == 'X' || tabella[0][1] == 'O'){
        return true;
    }else if tabella[0][2] == tabella[1][2] && tabella[1][2] == tabella[2][2] && (tabella[0][2] == 'X' || tabella[0][2] == 'O'){
        return true;
    }else if tabella[0][0] == tabella[1][1] && tabella[2][2] == tabella[1][1] && (tabella[0][0] == 'X' || tabella[0][0] == 'O'){
        return true;
    }else if tabella[0][2] == tabella[1][1] && tabella[1][1] == tabella[2][0] && (tabella[0][2] == 'X' || tabella[0][2] == 'O'){
        return  true;
    }else {
        return false;
    }
}